use super::board_representation::{Bitboard, Square, NUM_SQUARES};
pub type Magic = u64;

const NUM_HASH_ENTRIES: usize = 107648;

#[derive(Debug, Copy, Clone)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: Magic,
    pub shift: u8,
    pub offset: usize,
}

impl MagicEntry {
    pub const fn new(mask: Bitboard, magic: Magic, shift: u8, offset: usize) -> Self {
        Self {
            mask,
            magic,
            shift,
            offset,
        }
    }

    const fn hash_index(self, blockers: Bitboard) -> usize {
        (((blockers.as_u64().wrapping_mul(self.magic)) >> self.shift) as usize) + self.offset
    }
}

#[derive(Debug)]
pub struct MagicLookup {
    rook_entries: [MagicEntry; NUM_SQUARES as usize],
    bishop_entries: [MagicEntry; NUM_SQUARES as usize],
    hash_table: [Bitboard; NUM_HASH_ENTRIES],
}

impl MagicLookup {
    pub const fn new(
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

    const fn attacks_from_hash(&self, entry: &MagicEntry, occupied: Bitboard) -> Bitboard {
        let blockers = occupied.intersection(entry.mask);
        self.hash_table[entry.hash_index(blockers)]
    }

    pub const fn bishop_attack_set(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        let entry = &self.bishop_entries[sq.as_index()];
        self.attacks_from_hash(entry, occupied)
    }

    pub const fn rook_attack_set(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        let entry = &self.rook_entries[sq.as_index()];
        self.attacks_from_hash(entry, occupied)
    }
}
