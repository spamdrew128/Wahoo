use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    board::chess_move::Move,
    board::zobrist::ZobristHash,
    eval::evaluation::{EvalScore, MATE_THRESHOLD, TB_LOSS_SCORE, TB_WIN_SCORE},
    search::search::{Depth, Ply},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct TTFlag(u8);

impl TTFlag {
    pub const UNINITIALIZED: Self = Self(0b00 << 6);
    pub const LOWER_BOUND: Self = Self(0b01 << 6);
    pub const EXACT: Self = Self(0b10 << 6);
    pub const UPPER_BOUND: Self = Self(0b11 << 6);

    const fn new(data: u8) -> Self {
        Self(data)
    }

    pub const fn determine(
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct AgeAndFlag(u8);
impl AgeAndFlag {
    const AGE_BITFIELD: u8 = 0b00111111;
    const FLAG_BITFIELD: u8 = 0b11000000;

    const fn new(age: u8, flag: TTFlag) -> Self {
        Self(age | flag.0)
    }

    const fn flag(self) -> TTFlag {
        TTFlag::new(self.0 & Self::FLAG_BITFIELD)
    }

    const fn age(self) -> u8 {
        self.0 & Self::AGE_BITFIELD
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct TTEntry {
    age_and_flag: AgeAndFlag, // 1 byte
    depth: Depth,             // 1 byte
    pub best_move: Move,      // 2 byte
    score: i16,               // 2 byte
    key: u16,                 // 2 byte
}

impl TTEntry {
    const BYTES: usize = 8;

    const fn new(
        age: u8,
        flag: TTFlag,
        depth: Depth,
        best_move: Move,
        score: i16,
        key: u16,
    ) -> Self {
        let age_and_flag = AgeAndFlag::new(age, flag);
        Self {
            age_and_flag,
            depth,
            best_move,
            score,
            key,
        }
    }

    const fn key_from_hash(hash: ZobristHash) -> u16 {
        // use upper 16 bits for key
        (hash.as_u64() >> 48) as u16
    }

    fn score_to_tt(score: EvalScore, ply: Ply) -> i16 {
        // Adjust to be relative to the node, rather than relative to the position
        if score >= TB_WIN_SCORE {
            (score as i16) + i16::from(ply)
        } else if score <= TB_LOSS_SCORE {
            (score as i16) - i16::from(ply)
        } else {
            score as i16
        }
    }

    pub fn score_from_tt(self, ply: Ply) -> EvalScore {
        let score = i32::from(self.score);
        if score >= MATE_THRESHOLD {
            score - i32::from(ply)
        } else if score <= -MATE_THRESHOLD {
            score + i32::from(ply)
        } else {
            score
        }
    }

    pub fn cutoff_is_possible(
        self,
        alpha: EvalScore,
        beta: EvalScore,
        current_depth: Depth,
    ) -> bool {
        if self.depth < current_depth {
            return false;
        }

        let score = i32::from(self.score);
        match self.age_and_flag.flag() {
            TTFlag::EXACT => true,
            TTFlag::LOWER_BOUND => score >= beta,
            TTFlag::UPPER_BOUND => score <= alpha,
            _ => false,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    const fn quality(self) -> u16 {
        let age = self.age_and_flag.age() as u16;
        let depth = self.depth as u16;
        age * 2 + depth
    }
}

impl From<u64> for TTEntry {
    fn from(data: u64) -> Self {
        // SAFETY: This is safe because all fields of TTEntry are (at base) integral types, and order is known.
        unsafe { std::mem::transmute(data) }
    }
}

impl From<TTEntry> for u64 {
    fn from(entry: TTEntry) -> Self {
        // SAFETY: This is safe because all bitpatterns of `u64` are valid.
        unsafe { std::mem::transmute(entry) }
    }
}

#[derive(Debug)]
pub struct TranspositionTable {
    table: Vec<AtomicU64>,
    age: u8,
}

impl TranspositionTable {
    pub fn new(megabytes: usize) -> Self {
        const BYTES_PER_MB: usize = 1024 * 1024;

        let bytes = megabytes * BYTES_PER_MB;
        let entries = bytes / TTEntry::BYTES;
        let mut table = vec![];
        table.resize_with(entries, AtomicU64::default);

        Self { table, age: 0 }
    }

    fn table_index(&self, hash: ZobristHash) -> usize {
        // use lower bits for index
        hash.as_usize() % self.table.len()
    }

    pub fn store(
        &self,
        flag: TTFlag,
        best_score: EvalScore,
        hash: ZobristHash,
        ply: Ply,
        depth: Depth,
        best_move: Move,
    ) {
        let score = TTEntry::score_to_tt(best_score, ply);
        let key = TTEntry::key_from_hash(hash);
        let mut new_entry = TTEntry::new(self.age, flag, depth, best_move, score, key);

        let index = self.table_index(hash);
        let old_entry: TTEntry = self.table[index].load(Ordering::Relaxed).into();

        if new_entry.quality() >= old_entry.quality() {
            if best_move.is_null() && new_entry.key == old_entry.key {
                new_entry.best_move = old_entry.best_move;
            }
            self.table[index].store(new_entry.into(), Ordering::Relaxed);
        }
    }

    pub fn probe(&self, hash: ZobristHash) -> Option<TTEntry> {
        let index = self.table_index(hash);
        let key = TTEntry::key_from_hash(hash);
        let entry = TTEntry::from(self.table[index].load(Ordering::Relaxed));

        if (entry.key == key) && (entry.age_and_flag.flag() != TTFlag::UNINITIALIZED) {
            Some(entry)
        } else {
            None
        }
    }

    pub fn hashfull(&self) -> i32 {
        let mut hash_full = 0;
        self.table.iter().take(1000).for_each(|x| {
            let entry = TTEntry::from(x.load(Ordering::Relaxed));
            if entry.age_and_flag.flag() != TTFlag::UNINITIALIZED {
                hash_full += 1;
            }
        });

        hash_full
    }

    pub fn age_table(&mut self) {
        const AGE_MAX: u8 = 63; // max value we can fit into 6 bits

        assert!(self.age <= AGE_MAX, "TT AGE EXCEEDED AGE_MAX");
        if self.age == AGE_MAX {
            self.age = 0;
            self.table.iter_mut().for_each(|x| {
                let mut entry = TTEntry::from(x.load(Ordering::Relaxed));
                let flag = entry.age_and_flag.flag();
                entry.age_and_flag = AgeAndFlag::new(0, flag);
                x.store(entry.into(), Ordering::Relaxed);
            });
        }

        self.age += 1;
    }

    pub fn reset(&mut self) {
        self.table
            .iter_mut()
            .for_each(|x| *x = AtomicU64::default());
        self.age = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{
        board_representation::{Board, START_FEN},
        chess_move::Move,
        zobrist::ZobristHash,
    };

    use super::{AgeAndFlag, TTEntry, TTFlag, TranspositionTable};

    #[test]
    fn probe_works() {
        let mut tt = TranspositionTable::new(16);
        tt.age_table();
        let board = Board::from_fen(START_FEN);
        let best_score = 16;
        let flag = TTFlag::EXACT;
        let hash = ZobristHash::complete(&board);
        let mv = Move::from_string("d2d4", &board);
        tt.store(flag, best_score, hash, 4, 4, mv);

        let entry = tt.probe(hash).unwrap();
        let expected = TTEntry::new(
            1,
            flag,
            4,
            mv,
            best_score.try_into().unwrap(),
            TTEntry::key_from_hash(hash),
        );
        assert_eq!(entry, expected);

        let other_board =
            Board::from_fen("r3k2r/ppp2ppp/2n1bn2/8/2P1N3/1P4P1/P3PPBP/bNBR2K1 w kq - 0 12");
        let other_hash = ZobristHash::complete(&other_board);
        assert_eq!(tt.probe(other_hash), None);
    }

    #[test]
    fn flag_packing() {
        let age = 43;
        let flag = TTFlag::EXACT;
        let packed = AgeAndFlag::new(age, flag);
        assert_eq!(packed.age(), age);
        assert_eq!(packed.flag(), flag);
    }
}
