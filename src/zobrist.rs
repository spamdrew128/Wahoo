use crate::{
    bitloop,
    board_representation::{Board, Color, Piece, Square, NUM_COLORS, NUM_PIECES, NUM_SQUARES},
};

#[derive(Debug, Copy, Clone)]
struct ZobristHash(u64);

impl ZobristHash {
    pub const fn as_index(self) -> usize {
        self.0 as usize
    }
}

const NUM_CASTLING_CONFIGURATIONS: usize = 16;
const NUM_FILES: usize = 2;
struct ZobristKeys {
    pieces: [[[u64; NUM_SQUARES as usize]; NUM_PIECES as usize]; NUM_COLORS as usize],
    castling: [u64; NUM_CASTLING_CONFIGURATIONS],
    ep_file: [u64; NUM_FILES],
    side_to_move: [u64; NUM_COLORS as usize],
}

const ZOBRIST_KEYS: ZobristKeys = include!(concat!(env!("OUT_DIR"), "/zobrist_keys_init.txt"));

fn hash_position(board: &Board) -> ZobristHash {
    let mut hash: u64 = 0;

    for color in Color::LIST {
        for piece in Piece::LIST {
            let mut piece_bb = board.piece_bb(piece, color);
            bitloop!(|sq|, piece_bb, {
                hash ^= ZOBRIST_KEYS.pieces[color.as_index()][piece.as_index()][sq.as_index()];
            });
        }
    }

    hash ^= ZOBRIST_KEYS.castling[board.castle_rights.as_index()];

    if let Some(ep_sq) = board.ep_sq {
        let file = usize::from(ep_sq.as_u16() % 8);
        hash ^= ZOBRIST_KEYS.ep_file[file];
    }

    hash ^= ZOBRIST_KEYS.side_to_move[board.color_to_move.as_index()];

    ZobristHash(hash)
}
