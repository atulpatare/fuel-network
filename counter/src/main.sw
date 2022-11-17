contract;

dep errors;
dep r#abi;

use errors::MyError;
use abi::Counter;

storage {
    counter: u64 = 0,
}

impl Counter for Contract {
    #[storage(read)]
    fn count() -> u64 {
        storage.counter
    }

    #[storage(read, write)]
    fn increment() {
        storage.counter += 1;
    }

    #[storage(read, write)]
    fn increment_custom(value: u64) {
        require(value > 0, MyError::Zero);
        storage.counter += value;
    }

    #[storage(read, write)]
    fn decrement() {
        storage.counter -= 1;
    }

    #[storage(read, write)]
    fn decrement_custom(value: u64) {
        require(value > 0, MyError::Zero);
        require(storage.counter - value > 0, MyError::Zero);
        storage.counter -= value;
    }
}