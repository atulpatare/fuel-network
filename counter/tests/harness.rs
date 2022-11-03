use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/counter-abi.json");

async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
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

#[tokio::test]
async fn can_get_contract_id() {
    let (instance, _id) = get_contract_instance().await;

    println!(
        "Initial count {}", 
        instance.methods().count().call().await.unwrap().value
    );

    // Increment the counter
    let _result = instance.methods().increment().call().await.unwrap();

    // Get the current value of the counter
    let result = instance.methods().count().call().await.unwrap();

    // Check that the current value of the counter is 1.
    // Recall that the initial value of the counter was 0.
    assert_eq!(result.value, 1);
    // let (instance, _id) = get_contract_instance().await;

    // let _result = instance.methods().increment().call().await.unwrap();
    // let result = instance.methods().count().call().await.unwrap();
    // println!("{}", result.value);
    // assert_eq!(result.value, 1);

    // let _decrement = instance.methods().increment().call().await.unwrap();
    // let decrement = instance.methods().count().call().await.unwrap();
    // assert_eq!(decrement.value, 0);
}
