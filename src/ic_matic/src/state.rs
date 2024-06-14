use candid::Principal;
use ethereum_types::Address;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use std::{cell::RefCell, collections::{BTreeMap, BTreeSet, HashSet}, fmt::{Display, Formatter}};

use crate::{
    events_utils::{EventSource, ReceivedPolygonEvent},
    evm_rpc_canister::BlockTag,
    numeric::{BlockNumber, LedgerMintIndex, Wei},
    rpc_providers::PolygonNetwork,
};

thread_local! {
    pub static STATE: RefCell<Option<State>> = RefCell::default();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MintedEvent {
    pub deposit_event: ReceivedPolygonEvent,
    pub mint_block_index: LedgerMintIndex,
    pub token_symbol: String,
}

impl MintedEvent {
    pub fn source(&self) -> EventSource {
        self.deposit_event.source()
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum InvalidStateError {
    InvalidTransactionNonce(String),
    InvalidEcdsaKeyName(String),
    InvalidLedgerId(String),
    InvalidEthereumContractAddress(String),
    InvalidErc20HelperContractAddress(String),
    InvalidMinimumWithdrawalAmount(String),
    InvalidLastScrapedBlockNumber(String),
    InvalidLastErc20ScrapedBlockNumber(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum InvalidEventReason {
    /// Deposit is invalid and was never minted.
    /// This is most likely due to a user error (e.g., user's IC principal cannot be decoded)
    /// or there is a critical issue in the logs returned from the JSON-RPC providers.
    InvalidDeposit(String),

    /// Deposit is valid but it's unknown whether it was minted or not,
    /// most likely because there was an unexpected panic in the callback.
    /// The deposit is quarantined to avoid any double minting and
    /// will not be further processed without manual intervention.
    QuarantinedDeposit,
}

impl Display for InvalidEventReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidEventReason::InvalidDeposit(reason) => {
                write!(f, "Invalid deposit: {}", reason)
            }
            InvalidEventReason::QuarantinedDeposit => {
                write!(f, "Quarantined deposit")
            }
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum TaskType {
    Mint,
    RetrieveEth,
    ScrapEthLogs,
    RefreshGasFeeEstimate,
    Reimbursement,
    MintCkErc20,
}

pub struct State {
    pub polygon_network: PolygonNetwork,
    pub ecdsa_key_name: String,
    pub icmatic_ledger_id: Principal,
    pub eth_helper_contract_address: Option<Address>,
    pub ecdsa_public_key: Option<EcdsaPublicKeyResponse>,
    pub icmatic_minimum_withdrawal_amount: Wei,
    pub ethereum_block_height: BlockTag,
    pub first_scraped_block_number: BlockNumber,
    pub last_scraped_block_number: BlockNumber,
    pub last_erc20_scraped_block_number: BlockNumber,
    pub last_observed_block_number: Option<BlockNumber>,
    pub events_to_mint: BTreeMap<EventSource, ReceivedPolygonEvent>,
    pub minted_events: BTreeMap<EventSource, MintedEvent>,
    pub invalid_events: BTreeMap<EventSource, InvalidEventReason>,
    // pub eth_transactions: EthTransactions,
    pub skipped_blocks: BTreeSet<BlockNumber>,
    /// Current balance of matic held by the minter.
    /// Computed based on audit events.
    pub matic_balance: ethnum::u256,

    /// Per-principal lock for pending withdrawals
    // pub pending_withdrawal_principals: BTreeSet<Principal>,

    /// Locks preventing concurrent execution timer tasks
    pub active_tasks: HashSet<TaskType>,

    /// Number of HTTP outcalls since the last upgrade.
    /// Used to correlate request and response in logs.
    pub http_request_counter: u64,
    // pub last_transaction_price_estimate: Option<(u64, GasFeeEstimate)>,
}


