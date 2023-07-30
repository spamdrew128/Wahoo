use crate::board::{
    attacks,
    board_representation::{Board, Color, Piece, NUM_PIECES},
    chess_move::{Flag, Move},
};

pub const SEE_VALS: [i32; (NUM_PIECES + 1) as usize] = [450, 450, 650, 1250, 100, 0, 0];

pub const ASCENDING_PIECE_ORDER: [Piece; NUM_PIECES as usize] = [
    Piece::PAWN,
    Piece::KNIGHT,
    Piece::BISHOP,
    Piece::ROOK,
    Piece::QUEEN,
    Piece::KING,
];

impl Board {
    pub fn see(&self, mv: Move, attacker: Piece, victim: Piece, threshold: i32) -> bool {
        let sq = mv.to();
        let mut color = self.color_to_move;
        let mut next = attacker;
        let mut occ = self.occupied() ^ sq.as_bitboard() ^ mv.from().as_bitboard();

        let base = if mv.flag() == Flag::EP {
            occ ^= sq.row_swap().as_bitboard();

            0
        } else if mv.is_promo() {
            next = mv.promo_piece();

            SEE_VALS[victim.as_index()] + SEE_VALS[mv.promo_piece().as_index()]
                - SEE_VALS[Piece::PAWN.as_index()]
        } else {
            SEE_VALS[victim.as_index()] - SEE_VALS[attacker.as_index()]
        };

        let mut score = base - threshold;

        // if we captured a higher value piece than we attacked with,
        // we have positive SEE no matter what
        if score >= 0 {
            return true;
        }

        let rooks = self.pieces[Piece::ROOK.as_index()];
        let bishops = self.pieces[Piece::BISHOP.as_index()];
        let queens = self.pieces[Piece::QUEEN.as_index()];

        let hv_sliders = rooks | queens;
        let d_sliders = bishops | queens;

        let mut all_attackers = (attacks::knight(sq) & self.pieces[Piece::KNIGHT.as_index()])
            | (attacks::king(sq) & self.pieces[Piece::KING.as_index()])
            | (attacks::rook(sq, occ) & hv_sliders)
            | (attacks::bishop(sq, occ) & d_sliders)
            | (attacks::pawn(sq, Color::White) & self.piece_bb(Piece::PAWN, Color::Black))
            | (attacks::pawn(sq, Color::Black) & self.piece_bb(Piece::PAWN, Color::White));

        color = color.flip();
        loop {
            let color_bb = self.all[color.as_index()];
            let our_attackers = all_attackers & color_bb;

            if our_attackers.is_empty() {
                break;
            }

            for piece in ASCENDING_PIECE_ORDER {
                let piece_bb = our_attackers & self.pieces[piece.as_index()];
                if piece_bb.is_not_empty() {
                    occ ^= piece_bb.lsb_bb();
                    next = piece;
                    break;
                }
            }

            if next == Piece::PAWN || next == Piece::BISHOP || next == Piece::QUEEN {
                all_attackers |= attacks::bishop(sq, occ) & bishops;
            }

            if next == Piece::ROOK || next == Piece::QUEEN {
                all_attackers |= attacks::rook(sq, occ) & rooks;
            }

            all_attackers = occ & all_attackers;
            score = -score - 1 - SEE_VALS[next.as_index()];
            color = color.flip();

            if score >= 0 {
                let our_defenders = all_attackers.intersection(self.all[color.as_index()]);
                // if the square is still defended, the king can't take and the capture chain ends
                if next == Piece::KING && our_defenders.is_not_empty() {
                    color = color.flip();
                }
                break;
            }
        }

        color != self.color_to_move
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{
        board_representation::{Board, Piece},
        chess_move::Move,
    };

    #[test]
    fn equal_position_see() {
        let board =
            Board::from_fen("rnbqkb1r/ppp1pppp/5n2/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 2 3");
        let mv = Move::from_string("c3d5", &board);

        assert!(board.see(mv, Piece::KNIGHT, Piece::PAWN, 0));
        assert!(!board.see(mv, Piece::KNIGHT, Piece::PAWN, 1));
    }

    #[test]
    fn ep_xray() {
        let board =
            Board::from_fen("1nbqkb1r/1pp1p3/5p1p/p2n2pP/4p3/P1N2Pr1/1PPP2P1/R1BQKBNR w k g6 0 16");
        let mv = Move::from_string("h5g6", &board);

        assert!(board.see(mv, Piece::PAWN, Piece::NONE, 0));
        assert!(!board.see(mv, Piece::PAWN, Piece::NONE, 1));
    }

    #[test]
    fn king_cant_end_chain() {
        let board = Board::from_fen("8/3b4/8/5nk1/8/5R2/K4R2/5R2 w - - 0 1");
        let mv = Move::from_string("f3f5", &board);

        assert!(board.see(mv, Piece::ROOK, Piece::KNIGHT, 0));
    }
}
