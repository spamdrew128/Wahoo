use crate::board_representation::{Bitboard, NUM_SQUARES};
pub type Magic = u64;

const NUM_HASH_ENTRIES: usize = 107648;

#[derive(Debug, Copy, Clone)]
struct MagicEntry {
    mask: Bitboard,
    magic: Magic,
    shift: u8,
    offset: usize,
}

impl MagicEntry {
    const fn hash_index(self, blockers: Bitboard) -> usize {
        ((blockers.as_u64().wrapping_mul(self.magic)) >> self.shift) as usize
    }
}

#[derive(Debug)]
pub struct MagicLookup {
    rook_entries: [MagicEntry; NUM_SQUARES as usize],
    bishop_entries: [MagicEntry; NUM_SQUARES as usize],
    hash_table: [Bitboard; NUM_HASH_ENTRIES],
}