mod address;
mod ecdsa_secp256k1;
mod edsa;
mod eth_types;
mod ic_crypto_sha3;
use crate::address::ecdsa_public_key_to_address;
use crate::eth_types::Address;
use candid::CandidType;
use ecdsa_secp256k1::PublicKey;
use edsa::get_public_key;
use ic_cdk::{
    api::management_canister::ecdsa::{ecdsa_public_key, EcdsaPublicKeyResponse},
    post_upgrade, pre_upgrade, println, query, update,
};

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
async fn getEthAddress() -> String {
    // get treshold ecdsa public key
    let ecdsa_public_key = get_public_key().await;
    let ethAddress = ecdsa_public_key_to_address(&ecdsa_public_key);
    println!("Public key{:?}", ethAddress);

    return ethAddress.to_string();
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we only use the k256 crate for verifying secp256k1 signatures, and such
// signature verification does not require any randomness.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

ic_cdk::export_candid!();
