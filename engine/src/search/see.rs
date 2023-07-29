use crate::board::{
    board_representation::{Board, Piece, NUM_PIECES},
    chess_move::{Flag, Move},
};

pub const SEE_VALS: [i32; (NUM_PIECES + 1) as usize] = [450, 450, 650, 1250, 100, 0, 0];

impl Move {
    fn see(self, board: &Board, attacker: Piece, victim: Piece, threshold: i32) {
        let mut piece = attacker;
        let mut val = SEE_VALS[victim.as_index()] - threshold;

        val += if self.flag() == Flag::EP {
            SEE_VALS[Piece::PAWN.as_index()]
        } else if self.is_promo() {
            piece = self.promo_piece();
            SEE_VALS[victim.as_index()] + SEE_VALS[self.promo_piece().as_index()]
                - SEE_VALS[Piece::PAWN.as_index()]
        } else {
            SEE_VALS[victim.as_index()]
        };
    }
}
