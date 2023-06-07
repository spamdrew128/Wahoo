use crate::board_representation::{NUM_COLORS, NUM_PIECES, NUM_SQUARES};

const NUM_CASTLING_CONFIGURATIONS: usize = 16;
const NUM_FILES: usize = 2;
struct ZobristKeys {
    pieces: [[[u64; NUM_SQUARES as usize]; NUM_PIECES as usize]; NUM_COLORS as usize],
    castling: [u64; NUM_CASTLING_CONFIGURATIONS],
    ep_file: [u64; NUM_FILES],
    side_to_move: [u64; NUM_COLORS as usize],
}
