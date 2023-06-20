// use std::sync::atomic::{AtomicU64, Ordering};

use crate::tuple_constants_enum;

#[derive(Debug, PartialEq, Eq)]
struct TTFlag(u8);

impl TTFlag {
    tuple_constants_enum!(Self,
        UNINITIALIZED,
        LOWER_BOUND,
        EXACT,
        UPPER_BOUND
    );

    const fn new(data: u8) -> Self {
        Self(data)
    }
}

struct TTEntry {

}

struct TranspositionTable {

}