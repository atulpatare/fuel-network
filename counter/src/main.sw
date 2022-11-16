contract;

use std::{address::Address, hash::sha256};

struct Count {
    id: u64,
    count: u64,
}

abi Counter {
    fn count() -> Count;
}

impl Counter for Contract {
    fn count() -> Count {
        Count{ id: 1, count: 1 }
    }
}
