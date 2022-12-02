contract;

dep errors;
dep interface;

use std::logging::log;

use errors::MyError;
use interface::{Counter, IncrementEvent, DecrementEvent};

storage {
    counter: u64 = 0,
}

impl Counter for Contract {
    #[storage(read)]
    fn count() -> u64 {
        storage.counter
    }

    #[storage(read, write)]
    fn increment(num: u64) {
        storage.counter += num;
        log(IncrementEvent {
            id: num,
            count: num,
        });
    }

    #[storage(read, write)]
    fn decrement(num: u64) {
        require(num < storage.counter, MyError::CannotDecrement);
        storage.counter -= num;
        log(DecrementEvent {
            id: num,
            count: num,
        })
    }
}
