mod events_utils;
mod log_types;
pub mod numeric;
mod polygon_rpc_clinet;
mod rpc_providers;
mod state;
use events_utils::ReceivedPolygonEvent;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use polygon_rpc_clinet::evm_rpc_canister::EthSepoliaService;
use polygon_rpc_clinet::evm_rpc_canister::MultiGetLogsResult;
use polygon_rpc_clinet::evm_rpc_canister::{
    BlockTag, EmvRpcService, GetLogsArgs, GetLogsResult, RpcApi, RpcConfig, RpcError, RpcService,
    RpcServices,
};
use polygon_rpc_clinet::EthRpcClient;
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

#[query]
async fn get_cycles(resposne_estimate_size: u64) -> u128 {
    let rpc_clinet = EthRpcClient {};
    // return rpc_clinet::estimate_cycles(resposne_estimate_size);
    let effective = rpc_clinet.effective_size_estimate(resposne_estimate_size);
    let cycles = rpc_clinet.estimate_cycles(effective);
    return cycles;
}

#[update]
async fn get_block(
    cycles: u128,
) -> Result<
    (polygon_rpc_clinet::evm_rpc_canister::MultiGetBlockByNumberResult,),
    (RejectionCode, String),
> {
    let sepolia_services: RpcServices =
        RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy]));
    let rpc_config = RpcConfig {
        responseSizeEstimate: Some(24 * 1204),
    };
    let block_tag = BlockTag::Finalized;
    let block: Result<
        (polygon_rpc_clinet::evm_rpc_canister::MultiGetBlockByNumberResult,),
        (RejectionCode, String),
    > = EmvRpcService
        .eth_get_block_by_number(sepolia_services, Some(rpc_config), block_tag, cycles)
        .await;
    return block;
}
ic_cdk::export_candid!();
