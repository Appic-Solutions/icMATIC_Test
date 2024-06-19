use candid::{CandidType, Principal};
use minicbor::{Decode, Encode};
use serde::Serialize;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

use crate::{
    eth_types::{address::Address, data::FixedSizeData, hash::Hash},
    numeric::checked_amount::CheckedAmountOf,
    numeric::{BlockNumber, LogIndex, Wei},
    polygon_rpc_clinet::evm_rpc_canister::LogEntry,
};

/// A unique identifier of the event source: the source transaction hash and the log
/// entry index.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventSource {
    pub transaction_hash: Hash,
    pub log_index: LogIndex,
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}:{}", self.transaction_hash, self.log_index)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ReceivedPolygonEvent {
    pub transaction_hash: Hash,
    pub block_number: BlockNumber,
    pub log_index: LogIndex,
    pub from_address: Address,
    pub value: Wei,
    pub principal: Principal,
}

impl ReceivedPolygonEvent {
    pub fn source(&self) -> EventSource {
        EventSource {
            transaction_hash: self.transaction_hash,
            log_index: self.log_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivedEventError {
    PendingLogEntry,
    InvalidEventSource {
        source: EventSource,
        error: EventSourceError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventSourceError {
    // failed to decode principal from bytes {invalid_principal}
    InvalidPrincipal { invalid_principal: FixedSizeData },
    // invalid ReceivedEthEvent
    InvalidEvent(String),
}

impl TryFrom<LogEntry> for ReceivedPolygonEvent {
    type Error = ReceivedEventError;

    fn try_from(entry: LogEntry) -> Result<Self, Self::Error> {
        let block_number = entry
            .blockNumber
            .ok_or(ReceivedEventError::PendingLogEntry)?;
        let transaction_hash = entry
            .transactionHash
            .ok_or(ReceivedEventError::PendingLogEntry)?;
        let _transaction_indexSerialize = entry
            .transactionIndex
            .ok_or(ReceivedEventError::PendingLogEntry)?;
        let log_index = entry.logIndex.ok_or(ReceivedEventError::PendingLogEntry)?;
        let event_source = EventSource {
            transaction_hash: Hash::from_str(&transaction_hash).unwrap(),
            log_index: CheckedAmountOf::new(log_index),
        };

        if entry.removed {
            return Err(ReceivedEventError::InvalidEventSource {
                source: event_source,
                error: EventSourceError::InvalidEvent(
                    "this event has been removed from the chain".to_string(),
                ),
            });
        }

        let parse_address = |address: &FixedSizeData| -> Result<Address, ReceivedEventError> {
            Address::try_from(&address.0).map_err(|err| ReceivedEventError::InvalidEventSource {
                source: event_source,
                error: EventSourceError::InvalidEvent(format!(
                    "Invalid address in log entry: {}",
                    err
                )),
            })
        };

        let parse_principal = |principal: &FixedSizeData| -> Result<Principal, ReceivedEventError> {
            parse_principal_from_slice(&principal.0).map_err(|_err| {
                ReceivedEventError::InvalidEventSource {
                    source: event_source,
                    error: EventSourceError::InvalidPrincipal {
                        invalid_principal: principal.clone(),
                    },
                }
            })
        };

        // We have only one non-indexed data field for Polygon and that is the amount.
        let value_bytes = FixedSizeData::from_str(&entry.data).map_err(|data| {
            ReceivedEventError::InvalidEventSource {
                source: event_source,
                error: EventSourceError::InvalidEvent(format!(
                    "Invalid data length; expected 32-byte value, got {}",
                    hex::encode(data)
                )),
            }
        })?;

        // We have 3 indexed topics for ETH events: (hash, from_address, principal) and they need to be extracted,
        // TODO: Converts topics from String to FixedSizedData

        match entry.topics[0].as_str() {
            "0x4d84986cd718ed41155c024ee6c78a9396f89afed335ee4cb0713996744b49ee" => {
                if entry.topics.len() != 3 {
                    return Err(ReceivedEventError::InvalidEventSource {
                        source: event_source,
                        error: EventSourceError::InvalidEvent(format!(
                            "Expected 3 topics for ReceivedEth event, got {}",
                            entry.topics.len()
                        )),
                    });
                };
                let from_address =
                    parse_address(&FixedSizeData::from_str(&entry.topics[1]).unwrap())?;
                let principal =
                    parse_principal(&FixedSizeData::from_str(&entry.topics[1]).unwrap())?;
                Ok(ReceivedPolygonEvent {
                    transaction_hash: Hash::from_str(&transaction_hash).unwrap(),
                    block_number: BlockNumber::new(block_number),
                    log_index: LogIndex::new(log_index),
                    from_address,
                    value: Wei::from_be_bytes(value_bytes.0),
                    principal,
                }
                .into())
            }

            _ => Err(ReceivedEventError::InvalidEventSource {
                source: event_source,
                error: EventSourceError::InvalidEvent(format!(
                    "Expected either ReceivedPolygon, got {}",
                    entry.topics[0]
                )),
            }),
        }
    }
}

/// Decode a candid::Principal from a slice of at most 32 bytes
/// encoded as follows
/// - the first byte is the number of bytes in the principal
/// - the next N bytes are the principal
/// - the remaining bytes are zero
///
/// Any other encoding will return an error.
/// Some specific valid [`Principal`]s are also not allowed
/// since the decoded principal will be used to receive ckETH:
/// * the management canister principal
/// * the anonymous principal
///
/// This method MUST never panic (decode bytes from untrusted sources).
fn parse_principal_from_slice(slice: &[u8]) -> Result<Principal, String> {
    const ANONYMOUS_PRINCIPAL_BYTES: [u8; 1] = [4];

    if slice.is_empty() {
        return Err("slice too short".to_string());
    }
    if slice.len() > 32 {
        return Err(format!("Expected at most 32 bytes, got {}", slice.len()));
    }
    let num_bytes = slice[0] as usize;
    if num_bytes == 0 {
        return Err("management canister principal is not allowed".to_string());
    }
    if num_bytes > 29 {
        return Err(format!(
            "invalid number of bytes: expected a number in the range [1,29], got {num_bytes}",
        ));
    }
    if slice.len() < 1 + num_bytes {
        return Err("slice too short".to_string());
    }
    let (principal_bytes, trailing_zeroes) = slice[1..].split_at(num_bytes);
    if !trailing_zeroes
        .iter()
        .all(|trailing_zero| *trailing_zero == 0)
    {
        return Err("trailing non-zero bytes".to_string());
    }
    if principal_bytes == ANONYMOUS_PRINCIPAL_BYTES {
        return Err("anonymous principal is not allowed".to_string());
    }
    Principal::try_from_slice(principal_bytes).map_err(|err| err.to_string())
}
