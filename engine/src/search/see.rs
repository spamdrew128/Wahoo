use crate::board::{
    board_representation::{Board, Piece, NUM_PIECES},
    chess_move::{Flag, Move},
};

pub const SEE_VALS: [i32; (NUM_PIECES + 1) as usize] = [450, 450, 650, 1250, 100, 0, 0];

impl Move {
    fn see(self, board: &Board, attacker: Piece, victim: Piece, threshold: i32) -> bool {
        let sq = self.to();
        let mut val = -threshold;
        let mut next = attacker;
        let mut occ = board.occupied() ^ sq.as_bitboard() ^ self.from().as_bitboard();

        val += if self.flag() == Flag::EP {
            occ ^= sq.row_flip().as_bitboard();
            SEE_VALS[Piece::PAWN.as_index()]
        } else if self.is_promo() {
            next = self.promo_piece();
            SEE_VALS[victim.as_index()] + SEE_VALS[self.promo_piece().as_index()]
                - SEE_VALS[Piece::PAWN.as_index()]
        } else {
            SEE_VALS[victim.as_index()]
        };

        // if we captured a higher value piece than we attacked with,
        // we have positive SEE no matter what
        if val >= 0 {
            return true;
        }

        false
    }
}
