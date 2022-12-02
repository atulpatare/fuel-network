library interface;

pub struct IncrementEvent {
    id: u64,
    count: u64,
}

pub struct DecrementEvent {
    id: u64,
    count: u64,
}

abi Counter {
    #[storage(read)]
    fn count() -> u64;

    #[storage(read, write)]
    fn increment(num: u64);

    #[storage(read, write)]
    fn decrement(num: u64);
}
