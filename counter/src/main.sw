contract;

abi Counter {
    fn increment(num: u64) -> IncrementEvent;
    fn decrement(num: u64) -> DecrementEvent;
}

pub struct IncrementEvent {
    id: u64,
    count: u64,
}

pub struct DecrementEvent {
    id: u64,
    count: u64,
}

impl Counter for Contract {
    fn increment(num: u64) -> IncrementEvent {
        IncrementEvent {
            id: num,
            count: num,
        }
    }

    fn decrement(num: u64) -> DecrementEvent {
        DecrementEvent {
            id: num,
            count: num,
        }
    }
}
