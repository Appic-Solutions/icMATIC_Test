// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.

#![allow(non_snake_case)]
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

pub const CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai

#[derive(CandidType, Deserialize)]
pub enum Auth {
    RegisterProvider,
    FreeRpc,
    PriorityRpc,
    Manage,
}

#[derive(CandidType, Deserialize)]
pub enum EthSepoliaService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub enum L2MainnetService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Deserialize)]
pub enum EthMainnetService {
    Alchemy,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>),
    BaseMainnet(Option<Vec<L2MainnetService>>),
    Custom { chainId: u64, services: Vec<RpcApi> },
    OptimismMainnet(Option<Vec<L2MainnetService>>),
    ArbitrumOne(Option<Vec<L2MainnetService>>),
    EthMainnet(Option<Vec<EthMainnetService>>),
}

#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(u128),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistoryArgs {
    pub blockCount: u128,
    pub newestBlock: BlockTag,
    pub rewardPercentiles: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct FeeHistory {
    pub reward: Vec<Vec<u128>>,
    pub gasUsedRatio: Vec<f64>,
    pub oldestBlock: u128,
    pub baseFeePerGas: Vec<u128>,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum ProviderError {
    TooFewCycles { expected: u128, received: u128 },
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum ValidationError {
    CredentialPathNotAllowed,
    HostNotAllowed(String),
    CredentialHeaderNotAllowed,
    UrlParseError(String),
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Debug, Deserialize)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Debug, Deserialize)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize)]
pub enum FeeHistoryResult {
    Ok(Option<FeeHistory>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RpcService {
    EthSepolia(EthSepoliaService),
    BaseMainnet(L2MainnetService),
    Custom(RpcApi),
    OptimismMainnet(L2MainnetService),
    ArbitrumOne(L2MainnetService),
    EthMainnet(EthMainnetService),
    Chain(u64),
    Provider(u64),
}

#[derive(CandidType, Deserialize)]
pub enum MultiFeeHistoryResult {
    Consistent(FeeHistoryResult),
    Inconsistent(Vec<(RpcService, FeeHistoryResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct Block {
    pub miner: String,
    pub totalDifficulty: u128,
    pub receiptsRoot: String,
    pub stateRoot: String,
    pub hash: String,
    pub difficulty: u128,
    pub size: u128,
    pub uncles: Vec<String>,
    pub baseFeePerGas: u128,
    pub extraData: String,
    pub transactionsRoot: Option<String>,
    pub sha3Uncles: String,
    pub nonce: u128,
    pub number: u128,
    pub timestamp: u128,
    pub transactions: Vec<String>,
    pub gasLimit: u128,
    pub logsBloom: String,
    pub parentHash: String,
    pub gasUsed: u128,
    pub mixHash: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlockByNumberResult {
    Ok(Block),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBlockByNumberResult {
    Consistent(GetBlockByNumberResult),
    Inconsistent(Vec<(RpcService, GetBlockByNumberResult)>),
}

pub type Topic = Vec<String>;
#[derive(CandidType, Deserialize)]
pub struct GetLogsArgs {
    pub fromBlock: Option<BlockTag>,
    pub toBlock: Option<BlockTag>,
    pub addresses: Vec<String>,
    pub topics: Option<Vec<Topic>>,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct LogEntry {
    pub transactionHash: Option<String>,
    pub blockNumber: Option<u128>,
    pub data: String,
    pub blockHash: Option<String>,
    pub transactionIndex: Option<u128>,
    pub topics: Vec<String>,
    pub address: String,
    pub logIndex: Option<u128>,
    pub removed: bool,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum GetLogsResult {
    Ok(Vec<LogEntry>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetLogsResult {
    Consistent(GetLogsResult),
    Inconsistent(Vec<(RpcService, GetLogsResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionCountResult {
    Ok(u128),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionCountResult {
    Consistent(GetTransactionCountResult),
    Inconsistent(Vec<(RpcService, GetTransactionCountResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct TransactionReceipt {
    pub to: String,
    pub status: u128,
    pub transactionHash: String,
    pub blockNumber: u128,
    pub from: String,
    pub logs: Vec<LogEntry>,
    pub blockHash: String,
    pub r#type: String,
    pub transactionIndex: u128,
    pub effectiveGasPrice: u128,
    pub logsBloom: String,
    pub contractAddress: Option<String>,
    pub gasUsed: u128,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionReceiptResult {
    Ok(Option<TransactionReceipt>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionReceiptResult {
    Consistent(GetTransactionReceiptResult),
    Inconsistent(Vec<(RpcService, GetTransactionReceiptResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionStatus {
    Ok(Option<String>),
    NonceTooLow,
    NonceTooHigh,
    InsufficientFunds,
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionResult {
    Ok(SendRawTransactionStatus),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

pub type ProviderId = u64;
#[derive(CandidType, Deserialize)]
pub struct Metrics {
    pub cyclesWithdrawn: u128,
    pub responses: Vec<((String, String, String), u64)>,
    pub errNoPermission: u64,
    pub inconsistentResponses: Vec<((String, String), u64)>,
    pub cyclesCharged: Vec<((String, String), u128)>,
    pub requests: Vec<((String, String), u64)>,
    pub errHttpOutcall: Vec<((String, String), u64)>,
    pub errHostNotAllowed: Vec<(String, u64)>,
}

#[derive(CandidType, Deserialize)]
pub struct ProviderView {
    pub cyclesPerCall: u64,
    pub owner: Principal,
    pub hostname: String,
    pub primary: bool,
    pub chainId: u64,
    pub cyclesPerMessageByte: u64,
    pub providerId: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ManageProviderArgs {
    pub service: Option<RpcService>,
    pub primary: Option<bool>,
    pub providerId: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterProviderArgs {
    pub cyclesPerCall: u64,
    pub credentialPath: String,
    pub hostname: String,
    pub credentialHeaders: Option<Vec<HttpHeader>>,
    pub chainId: u64,
    pub cyclesPerMessageByte: u64,
}

#[derive(CandidType, Deserialize)]
pub enum RequestResult {
    Ok(String),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RequestCostResult {
    Ok(u128),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProviderArgs {
    pub cyclesPerCall: Option<u64>,
    pub credentialPath: Option<String>,
    pub hostname: Option<String>,
    pub credentialHeaders: Option<Vec<HttpHeader>>,
    pub primary: Option<bool>,
    pub cyclesPerMessageByte: Option<u64>,
    pub providerId: u64,
}

pub struct EmvRpcService;
impl EmvRpcService {
    pub async fn eth_get_logs(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: GetLogsArgs,
        cycles: u128,
    ) -> Result<(MultiGetLogsResult,)> {
        ic_cdk::api::call::call_with_payment128(
            CANISTER_ID,
            "eth_getLogs",
            (arg0, arg1, arg2),
            cycles,
        )
        .await
    }

    // pub async fn eth_fee_history(
    //     &self,
    //     arg0: RpcServices,
    //     arg1: Option<RpcConfig>,
    //     arg2: FeeHistoryArgs,
    // ) -> Result<(MultiFeeHistoryResult,)> {
    //     ic_cdk::call(CANISTER_ID, "eth_feeHistory", (arg0, arg1, arg2)).await
    // }
    pub async fn eth_get_block_by_number(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: BlockTag,
        cycles: u128,
    ) -> Result<(MultiGetBlockByNumberResult,)> {
        ic_cdk::api::call::call_with_payment128(
            CANISTER_ID,
            "eth_getBlockByNumber",
            (arg0, arg1, arg2),
            cycles,
        )
        .await
    }
    // pub async fn eth_get_transaction_count(
    //     &self,
    //     arg0: RpcServices,
    //     arg1: Option<RpcConfig>,
    //     arg2: GetTransactionCountArgs,
    // ) -> Result<(MultiGetTransactionCountResult,)> {
    //     ic_cdk::call(CANISTER_ID, "eth_getTransactionCount", (arg0, arg1, arg2)).await
    // }
    // pub async fn eth_get_transaction_receipt(
    //     &self,
    //     arg0: RpcServices,
    //     arg1: Option<RpcConfig>,
    //     arg2: String,
    // ) -> Result<(MultiGetTransactionReceiptResult,)> {
    //     ic_cdk::call(CANISTER_ID, "eth_getTransactionReceipt", (arg0, arg1, arg2)).await
    // }
    // pub async fn eth_send_raw_transaction(
    //     &self,
    //     arg0: RpcServices,
    //     arg1: Option<RpcConfig>,
    //     arg2: String,
    // ) -> Result<(MultiSendRawTransactionResult,)> {
    //     ic_cdk::call(CANISTER_ID, "eth_sendRawTransaction", (arg0, arg1, arg2)).await
    // }
    // pub async fn get_accumulated_cycle_count(&self, arg0: ProviderId) -> Result<(u128,)> {
    //     ic_cdk::call(CANISTER_ID, "getAccumulatedCycleCount", (arg0,)).await
    // }
    // pub async fn get_authorized(&self, arg0: Auth) -> Result<(Vec<Principal>,)> {
    //     ic_cdk::call(CANISTER_ID, "getAuthorized", (arg0,)).await
    // }
    // pub async fn get_metrics(&self) -> Result<(Metrics,)> {
    //     ic_cdk::call(CANISTER_ID, "getMetrics", ()).await
    // }
    // pub async fn get_nodes_in_subnet(&self) -> Result<(u32,)> {
    //     ic_cdk::call(CANISTER_ID, "getNodesInSubnet", ()).await
    // }
    // pub async fn get_open_rpc_access(&self) -> Result<(bool,)> {
    //     ic_cdk::call(CANISTER_ID, "getOpenRpcAccess", ()).await
    // }
    // pub async fn get_providers(&self) -> Result<(Vec<ProviderView>,)> {
    //     ic_cdk::call(CANISTER_ID, "getProviders", ()).await
    // }
    // pub async fn get_service_provider_map(&self) -> Result<(Vec<(RpcService, u64)>,)> {
    //     ic_cdk::call(CANISTER_ID, "getServiceProviderMap", ()).await
    // }
    // pub async fn manage_provider(&self, arg0: ManageProviderArgs) -> Result<()> {
    //     ic_cdk::call(CANISTER_ID, "manageProvider", (arg0,)).await
    // }
    // pub async fn register_provider(&self, arg0: RegisterProviderArgs) -> Result<(u64,)> {
    //     ic_cdk::call(CANISTER_ID, "registerProvider", (arg0,)).await
    // }
    // pub async fn request(
    //     &self,
    //     arg0: RpcService,
    //     arg1: String,
    //     arg2: u64,
    // ) -> Result<(RequestResult,)> {
    //     ic_cdk::call(CANISTER_ID, "request", (arg0, arg1, arg2)).await
    // }
    pub async fn request_cost(
        &self,
        arg0: RpcService,
        arg1: String,
        arg2: u64,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(CANISTER_ID, "requestCost", (arg0, arg1, arg2)).await
    }
    // pub async fn set_open_rpc_access(&self, arg0: bool) -> Result<()> {
    //     ic_cdk::call(CANISTER_ID, "setOpenRpcAccess", (arg0,)).await
    // }
    // pub async fn unregister_provider(&self, arg0: ProviderId) -> Result<(bool,)> {
    //     ic_cdk::call(CANISTER_ID, "unregisterProvider", (arg0,)).await
    // }
    // pub async fn update_provider(&self, arg0: UpdateProviderArgs) -> Result<()> {
    //     ic_cdk::call(CANISTER_ID, "updateProvider", (arg0,)).await
    // }
    // pub async fn withdraw_accumulated_cycles(
    //     &self,
    //     arg0: ProviderId,
    //     arg1: Principal,
    // ) -> Result<()> {
    //     ic_cdk::call(CANISTER_ID, "withdrawAccumulatedCycles", (arg0, arg1)).await
    // }
}
