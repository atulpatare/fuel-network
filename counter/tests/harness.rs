mod setup;

#[tokio::test]
async fn should_return_count() {
    let (instance, _id) = setup::get_contract_instance().await;
    let count = setup::count_fn(&instance).await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn should_increment_the_count() {
    let (instance, _id) = setup::get_contract_instance().await;
    let count = setup::count_fn(&instance).await;
    assert_eq!(count, 0);

    setup::increment_fn(&instance, 2).await;
    let updated = setup::count_fn(&instance).await;
    assert_eq!(updated, 2);
}

#[tokio::test]
#[should_panic(expected = "Revert(18446744073709486080)")]
async fn should_panic_on_decrement() {
    let (instance, _id) = setup::get_contract_instance().await;
    let count = setup::count_fn(&instance).await;
    assert_eq!(count, 0);

    setup::decrement_fn(&instance, 2).await;
}

#[tokio::test]
async fn should_decrement_the_count() {
    let (instance, _id) = setup::get_contract_instance().await;
    let count = setup::count_fn(&instance).await;
    assert_eq!(count, 0);

    setup::increment_fn(&instance, 2).await;
    setup::decrement_fn(&instance, 1).await;
    let updated = setup::count_fn(&instance).await;
    assert_eq!(updated, 1);
}
