use std::str::FromStr;

use fuels::prelude::{
    Bech32ContractId, Contract, Provider, TxParameters, WalletUnlocked,
};
use fuels::signers::fuel_crypto::SecretKey;
use fuels_abigen_macro::abigen;
use fuels_core::parameters::StorageConfiguration;

abigen!(
    Counter,
    "../counter/out/debug/counter-abi.json"
);

async fn get_contract_id(wallet: &WalletUnlocked) -> Bech32ContractId {
    let contract_id = Contract::deploy(
        "../counter/out/debug/counter.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../counter/out/debug/counter-storage_slots.json".to_string(),
        )),
    )
        .await
        .unwrap();

    contract_id
}

async fn setup_provider_and_wallet(port: u16) -> (Provider, WalletUnlocked) {
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
    let (_provider, wallet) = setup_provider_and_wallet(4000).await;
    let contract_id: Bech32ContractId = get_contract_id(&wallet).await;
    let contract: Counter = Counter::new(contract_id, wallet);

    let count = contract.methods().count().call().await.unwrap().value;
    let inc_res = contract.methods().increment(10).call().await;
    assert_eq!(inc_res.is_err(), false);

    let dec_res = contract.methods().decrement(5).call().await;
    assert_eq!(dec_res.is_err(), false);
    let updated = contract.methods().count().call().await.unwrap().value;

    println!("Count initial :: {}", count);
    println!("Count second :: {}", updated);

    Ok(())
}
