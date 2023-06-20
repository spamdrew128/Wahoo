use std::sync::atomic::AtomicU64;

use crate::{
    chess_move::Move,
    evaluation::{EvalScore, MATE_THRESHOLD},
    search::{Depth, Ply},
    tuple_constants_enum,
    zobrist::ZobristHash,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct TTFlag(u8);

impl TTFlag {
    tuple_constants_enum!(Self, UNINITIALIZED, LOWER_BOUND, EXACT, UPPER_BOUND);

    const fn new(data: u8) -> Self {
        Self(data)
    }

    const fn determine(
        best_score: EvalScore,
        old_alpha: EvalScore,
        alpha: EvalScore,
        beta: EvalScore,
    ) -> Self {
        if best_score >= beta {
            Self::LOWER_BOUND
        } else if alpha != old_alpha {
            Self::EXACT
        } else {
            Self::UPPER_BOUND
        }
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

    const fn cutoff_is_possible(
        self,
        alpha: EvalScore,
        beta: EvalScore,
        current_depth: Depth,
    ) -> bool {
        if self.depth < current_depth {
            return false;
        }

        match self.flag {
            TTFlag::EXACT => true,
            TTFlag::LOWER_BOUND => self.score >= beta,
            TTFlag::UPPER_BOUND => self.score <= alpha,
            _ => false,
        }
    }
}

impl From<AtomicU64> for TTEntry {
    fn from(data: AtomicU64) -> Self {
        // SAFETY: This is safe because all fields of TTEntry are (at base) integral types, and order is known.
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

    fn score_to_tt(score: EvalScore, ply: Ply) -> EvalScore {
        // Adjust to be relative to the node, rather than relative to the position
        if score >= MATE_THRESHOLD {
            score + i16::from(ply)
        } else if score <= -MATE_THRESHOLD {
            score - i16::from(ply)
        } else {
            score
        }
    }

    fn score_from_tt(score: EvalScore, ply: Ply) -> EvalScore {
        if score >= MATE_THRESHOLD {
            score - i16::from(ply)
        } else if score <= -MATE_THRESHOLD {
            score + i16::from(ply)
        } else {
            score
        }
    }

    const fn key_from_hash(hash: ZobristHash) -> u16 {
        // use upper 16 bits for key
        (hash.as_u64() >> 48) as u16
    }

    fn index(&self, hash: ZobristHash) -> usize {
        // use lower bits for index
        hash.as_usize() % self.table.len()
    }

    #[allow(clippy::too_many_arguments)]
    fn store(
        &self,
        best_score: EvalScore,
        old_alpha: EvalScore,
        alpha: EvalScore,
        beta: EvalScore,
        hash: ZobristHash,
        ply: Ply,
        depth: Depth,
        best_move: Move,
    ) {
        let flag = TTFlag::determine(best_score, old_alpha, alpha, beta);
        let score = Self::score_to_tt(best_score, ply);
        let key = Self::key_from_hash(hash);
        let entry = TTEntry::new(flag, depth, best_move, score, key);
        
        self.table[self.index(hash)]
    }
}
