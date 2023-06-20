use std::sync::atomic::{AtomicU64, Ordering};

use crate::{chess_move::Move, evaluation::EvalScore, search::Depth, tuple_constants_enum};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct TTFlag(u8);

impl TTFlag {
    tuple_constants_enum!(Self, UNINITIALIZED, LOWER_BOUND, EXACT, UPPER_BOUND);

    const fn new(data: u8) -> Self {
        Self(data)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct TTEntry {
    flag: TTFlag,     // 1 byte
    depth: Depth,     // 1 byte
    best_move: Move,  // 2 byte
    score: EvalScore, // 2 byte
    key: u16,         // 2 byte
}

impl TTEntry {
    const BYTES: usize = 8;

    const fn new(flag: TTFlag, depth: Depth, best_move: Move, score: EvalScore, key: u16) -> Self {
        Self {
            flag,
            depth,
            best_move,
            score,
            key,
        }
    }
}

impl From<AtomicU64> for TTEntry {
    fn from(data: AtomicU64) -> Self {
        // SAFETY: This is safe because all fields of TTEntry are (at base) integral types,
        unsafe { std::mem::transmute(data) }
    }
}

impl From<TTEntry> for AtomicU64 {
    fn from(entry: TTEntry) -> Self {
        // SAFETY: This is safe because all bitpatterns of `u64` are valid.
        unsafe { std::mem::transmute(entry) }
    }
}

struct TranspositionTable {
    table: Vec<AtomicU64>,
}

impl TranspositionTable {
    fn new(megabytes: usize) -> Self {
        const BYTES_PER_MB: usize = 1024 * 1024;

        let bytes = megabytes * BYTES_PER_MB;
        let entries = bytes / TTEntry::BYTES;
        let mut table = vec![];
        table.resize_with(entries, AtomicU64::default);

        Self { table }
    }

    fn reset(&mut self) {
        self.table
            .iter_mut()
            .for_each(|x| *x = AtomicU64::default());
    }
}
