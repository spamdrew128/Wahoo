// use std::sync::atomic::{AtomicU64, Ordering};

use crate::{tuple_constants_enum, chess_move::Move, search::Depth, evaluation::EvalScore};

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

#[repr(C)]
struct TTEntry {
    flag: TTFlag, // 1 byte
    depth: Depth, // 1 byte
    best_move: Move, // 2 byte
    score: EvalScore, // 2 byte
    key: u16, // 2 byte
}

impl From<u64> for TTEntry {
    fn from(data: u64) -> Self {
        // SAFETY: This is safe because all fields of TTEntry are (at base) integral types,
        unsafe { std::mem::transmute(data) }
    }
}

impl From<TTEntry> for u64 {
    fn from(entry: TTEntry) -> Self {
        // SAFETY: This is safe because all bitpatterns of `u64` are valid.
        unsafe { std::mem::transmute(entry) }
    }
}

struct TranspositionTable {

}