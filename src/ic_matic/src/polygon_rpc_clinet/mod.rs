pub mod evm_rpc_canister;

use crate::numeric::BlockNumber;
use evm_rpc_canister::{
    Block, BlockTag, EmvRpcService, EthSepoliaService, MultiGetBlockByNumberResult,
    RequestCostResult, RpcService, RpcServices,
};
use ic_cdk::trap;

const HEADER_SIZE_LIMIT: u64 = 2 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EthRpcClient {}

impl EthRpcClient {
    pub async fn estimate_cycles(&self, effective_size_estimate: u64) -> u128 {
        let sepolia_services: RpcService = RpcService::EthSepolia(EthSepoliaService::Alchemy);
        let cycles = EmvRpcService
            .request_cost(sepolia_services, " ".to_string(), effective_size_estimate)
            .await
            .expect("failed to get the cost");
        match cycles.0{
            RequestCostResult::Ok(needed_cycles) => needed_cycles,
            RequestCostResult::Err(error) => trap("failed to estimate number of cycles"),
        }

    }

    pub fn effective_size_estimate(&self, response_size_estimate: u64) -> u64 {
        return response_size_estimate + HEADER_SIZE_LIMIT;
    }
    // pub async fn eth_get_block_by_number(
    //     &self,
    //     block: BlockTag,
    // ) -> Result<BlockNumber, MultiGetBlockByNumberResult> {
    //     let expected_block_size = 24 * 1024;

    //     let results: MultiCallResults<Block> = self
    //         .parallel_call(
    //             "eth_getBlockByNumber",
    //             GetBlockByNumberParams {
    //                 block,
    //                 include_full_transactions: false,
    //             },
    //             ResponseSizeEstimate::new(expected_block_size),
    //         )
    //         .await;
    //     results.reduce_with_equality()
    // }
}
