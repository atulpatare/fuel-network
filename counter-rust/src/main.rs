#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use fuels_core::Identity;
use fuels::signers::fuel_crypto::SecretKey;
use fuels::prelude::{
    Bech32ContractId, Contract, Provider, TxParameters, WalletUnlocked,
};
use fuels_abigen_macro::abigen;
use fuels_core::parameters::StorageConfiguration;
use std::path::Path;
use std::str::FromStr;
extern crate rand; // 0.6.5

use rand::Rng;

pub fn tx_params() -> TxParameters {
    let gas_price = 0;
    let gas_limit = 1_000_000;
    let byte_price = 0;
    TxParameters::new(Some(gas_price), Some(gas_limit), Some(byte_price))
}

abigen!(
    Counter,
    "../counter/out/debug/counter-abi.json"
);

async fn get_contract_id(wallet: &WalletUnlocked) -> Bech32ContractId {
    debug!("Creating new deployment for non-existent contract");

    let _compiled =
        Contract::load_contract("../counter/out/debug/counter.bin", &None).unwrap();

    let bin_path = "../counter/out/debug/counter.bin".to_string();
    let contract_id = Contract::deploy(
        &bin_path,
        wallet,
        tx_params(),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    contract_id
}

async fn setup_provider_and_wallet(port: u16) -> (Provider, WalletUnlocked) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let address = format!("127.0.0.1:{}", port);
    let provider = Provider::connect(&address).await.unwrap();

    let secret = SecretKey::from_str(
        "de97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c",
    ).unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));

    (provider, wallet)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let (_provider, wallet) = setup_provider_and_wallet(4000).await;
    let contract_id: Bech32ContractId = get_contract_id(&wallet).await;
    info!("Using contract at {}", contract_id.to_string());
    let contract: Counter = Counter::new(contract_id, wallet);

    // sending random no to increment with
    let _ = contract.methods().increment(rand::thread_rng().gen_range(0..100)).call().await;
    let _ = contract.methods().decrement(rand::thread_rng().gen_range(0..100)).call().await;

    Ok(())
}
