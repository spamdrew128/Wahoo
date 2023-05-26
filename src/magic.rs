use crate::board_representation::{Bitboard, NUM_SQUARES};
pub type Magic = u64;

const NUM_HASH_ENTRIES: usize = 107648;

#[derive(Debug, Copy, Clone)]
struct MagicEntry {
    pub mask: Bitboard,
    pub magic: Magic,
    pub shift: u8,
    pub offset: usize,
}

impl MagicEntry {
    const fn new(mask: Bitboard, magic: Magic, shift: u8, offset: usize) -> Self {
        Self {
            mask,
            magic,
            shift,
            offset,
        }
    }

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

impl MagicLookup {
    const fn new(
        rook_entries: &[MagicEntry; NUM_SQUARES as usize],
        bishop_entries: &[MagicEntry; NUM_SQUARES as usize],
        hash_table: &[Bitboard; NUM_HASH_ENTRIES],
    ) -> Self {
        Self {
            rook_entries: *rook_entries,
            bishop_entries: *bishop_entries,
            hash_table: *hash_table,
        }
    }
}
