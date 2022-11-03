library r#abi;

abi Counter {
    #[storage(read, write)]
    fn increment();

    #[storage(read, write)]
    fn increment_custom(value: u64);

    #[storage(read, write)]
    fn decrement();

    #[storage(read, write)]
    fn decrement_custom(value: u64);

    #[storage(read)]
    fn count() -> u64;
}