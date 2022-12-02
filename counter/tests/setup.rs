use fuels::{prelude::*, tx::ContractId};
use fuels::contract::contract::CallResponse;
// Load abi from json
abigen!(MyContract, "out/debug/counter-abi.json");

pub async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/counter.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/counter-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

pub async fn increment_fn(instance: &MyContract, value: u64) -> CallResponse<()> {
    instance.methods().increment(value).call().await.unwrap()
}

pub async fn decrement_fn(instance: &MyContract, value: u64) -> CallResponse<()> {
    instance.methods().decrement(value).call().await.unwrap()
}

pub async fn count_fn(instance: &MyContract) -> u64 {
    instance.methods().count().simulate().await.unwrap().value
}
