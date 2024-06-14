mod checked_amount;
mod events_utils;
mod evm_rpc_canister;
mod log_types;
pub mod numeric;
mod rpc_providers;
mod state;
use candid::candid_method;
use candid::CandidType;
use events_utils::ReceivedPolygonEvent;
use evm_rpc_canister::EthSepoliaService;
use evm_rpc_canister::MultiGetLogsResult;
use evm_rpc_canister::{
    BlockTag, EmvRpcService, GetLogsArgs, GetLogsResult, RpcApi, RpcConfig, RpcError, RpcService,
    RpcServices,
};
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
// use minter::polygon_rpc_client::{providers, PolygonRPCWorker};

#[update]
async fn get_logs(cycles: u128) -> Result<String, String> {
    let sepolia_services: RpcServices =
        RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy]));
    let get_logs_args: GetLogsArgs = GetLogsArgs {
        fromBlock: Some(BlockTag::Number(272851)),
        toBlock: Some(BlockTag::Latest),
        addresses: vec!["0x0e2e8f489927b62725ae65ecb2c3ed410701a337".to_string()],
        topics: None,
    };
    let log_results = EmvRpcService
        .eth_get_logs(sepolia_services, None, get_logs_args, cycles)
        .await
        .unwrap();

    let mut log_entries: Vec<ReceivedPolygonEvent> = Vec::new();
    match log_results {
        (MultiGetLogsResult::Consistent(consistent_data),) => match consistent_data {
            GetLogsResult::Ok(logEntries) => {
                for logEntry in logEntries {
                    let new_log_entry: Result<
                        ReceivedPolygonEvent,
                        events_utils::ReceivedEventError,
                    > = ReceivedPolygonEvent::try_from(logEntry);
                    log_entries.push(new_log_entry.unwrap());
                }
                let json = to_string_pretty(&log_entries).unwrap();
                return Ok(json);
            }
            GetLogsResult::Err(error) => Err(format!("rpc error,{:?}", error).to_string()),
        },

        (MultiGetLogsResult::Inconsistent(inconsistent_data),) => {
            Ok("Inconsistent data".to_string())
        }
    }
}

ic_cdk::export_candid!();
