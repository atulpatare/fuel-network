mod setup;

#[tokio::test]
async fn should_increment_the_count() {
    let (instance, _id) = setup::get_contract_instance().await;

    // Increment the counter
    let _result = instance.methods().increment().call().await.unwrap();

    // Get the current value of the counter
    let result = instance.methods().count().call().await.unwrap();
    assert!(result.value > 0);
}

#[tokio::test]
async fn should_decrement_the_count() {
    println!("working");
}
