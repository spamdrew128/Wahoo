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

impl Move {
    fn see(self, board: &Board, attacker: Piece, victim: Piece, threshold: i32) -> bool {
        let sq = self.to();
        let mut color = board.color_to_move;
        let mut val = -threshold;
        let mut next = attacker;
        let mut occ = board.occupied() ^ sq.as_bitboard() ^ self.from().as_bitboard();

        val += if self.flag() == Flag::EP {
            occ ^= sq.row_swap().as_bitboard();
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

        let rooks = board.pieces[Piece::ROOK.as_index()];
        let bishops = board.pieces[Piece::BISHOP.as_index()];
        let queens = board.pieces[Piece::QUEEN.as_index()];

        let hv_sliders = rooks | queens;
        let d_sliders = bishops | queens;

        let mut all_attackers = (attacks::knight(sq) & board.pieces[Piece::KNIGHT.as_index()])
            | (attacks::king(sq) & board.pieces[Piece::KING.as_index()])
            | (attacks::rook(sq, occ) & hv_sliders)
            | (attacks::bishop(sq, occ) & d_sliders)
            | (attacks::pawn(sq, Color::White) & board.piece_bb(Piece::PAWN, Color::Black))
            | (attacks::pawn(sq, Color::Black) & board.piece_bb(Piece::PAWN, Color::White));

        loop {
            color = color.flip();
            let us = board.all[color.as_index()];
            let our_attackers = all_attackers & us;

            if our_attackers.is_empty() {
                break;
            }

            for piece in ASCENDING_PIECE_ORDER {
                let piece_bb = our_attackers & board.pieces[piece.as_index()];
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
        }

        false
    }
}
