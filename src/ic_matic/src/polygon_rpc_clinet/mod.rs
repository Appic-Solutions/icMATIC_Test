pub mod evm_rpc_canister;

use crate::numeric::BlockNumber;
use evm_rpc_canister::{
    Block, BlockTag, EmvRpcService, EthSepoliaService, MultiGetBlockByNumberResult,
    RequestCostResult, RpcConfig, RpcService, RpcServices,
};
use ic_cdk::trap;

const HEADER_SIZE_LIMIT: u64 = 2 * 1024;
pub const COLLATERAL_CYCLES_PER_NODE: u128 = 10_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolygonRpcClient {}

impl PolygonRpcClient {
    pub async fn estimate_cycles(
        &self,
        effective_size_estimate: u64,
        number_of_providers: u64,
    ) -> u128 {
        let sepolia_services: RpcService = RpcService::EthSepolia(EthSepoliaService::Alchemy);
        let cycles = EmvRpcService
            .request_cost(sepolia_services, " ".to_string(), effective_size_estimate)
            .await
            .expect("failed to get the cost");
        match cycles.0 {
            // https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/costs
            // there should be some addintional cycles passed as collateral cycles for futute additional costs
            RequestCostResult::Ok(required_cycles_per_call) => {
                (required_cycles_per_call + (COLLATERAL_CYCLES_PER_NODE * 28))
                    * number_of_providers as u128
            }
            RequestCostResult::Err(error) => trap("failed to estimate number of cycles"),
        }
    }

    pub fn effective_size_estimate(&self, response_size_estimate: u64) -> u64 {
        return response_size_estimate + HEADER_SIZE_LIMIT;
    }

    // pub async fn polygon_get_block_by_number(
    //     &self,
    //     block: BlockTag,
    // ) -> Result<BlockNumber, MultiGetBlockByNumberResult> {
    // let expected_block_size = 24 * 1024;

    // let sepolia_services: RpcServices =
    //     RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy]));
    // let rpc_config = RpcConfig {
    //     responseSizeEstimate: Some(24 * 1204),
    // };
    // let block_tag = BlockTag::Finalized;
    // let block: Result<
    //     (polygon_rpc_clinet::evm_rpc_canister::MultiGetBlockByNumberResult,),
    //     (RejectionCode, String),
    // > = EmvRpcService
    //     .eth_get_block_by_number(sepolia_services, Some(rpc_config), block_tag, cycles)
    //     .await;
    // return block;
    // }
}
