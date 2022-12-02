library r#abi;

pub struct ValueUpdated {
    counter: u64,
}

abi Counter {
    #[storage(read, write)]
    fn increment() -> ValueUpdated;

    #[storage(read, write)]
    fn increment_custom(value: u64);

    #[storage(read, write)]
    fn decrement();

    #[storage(read, write)]
    fn decrement_custom(value: u64);

    #[storage(read)]
    fn count() -> u64;
}