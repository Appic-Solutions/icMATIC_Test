mod evm_rpc_canister;
mod state;
mod log_types;
mod rpc_providers;
mod checked_amount;
pub mod numeric;
mod events_utils;
use candid::candid_method;
use candid::CandidType;
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
// use minter::polygon_rpc_client::{providers, PolygonRPCWorker};

#[update]
async fn get_logs(cycles: u128) -> Result<(MultiGetLogsResult,), (RejectionCode, String)> {
    let sepolia_services: RpcServices = RpcServices::Custom {
        chainId: 8002,
        services: vec![RpcApi {
            url: "".to_string(),
            headers: Some(vec![]),
        }],
    };  
    let get_logs_args: GetLogsArgs = GetLogsArgs {
        fromBlock: Some(BlockTag::Number(272851)),
        toBlock: Some(BlockTag::Latest),
        addresses: vec!["0x0e2e8f489927b62725ae65ecb2c3ed410701a337".to_string()],
        topics: None,
    };
    let log_results = EmvRpcService
        .eth_get_logs(sepolia_services, None, get_logs_args, cycles)
        .await;

    match log_results {
        Ok(data) => Ok(data),
        Err(error) => return Err(error),
    }
}
ic_cdk::export_candid!();
