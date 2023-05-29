use crate::attacks;
use crate::bitloop;
use crate::board_representation::{Bitboard, Board, Piece, Square};
use crate::chess_move::Move;
use crate::tuple_constants_enum;

macro_rules! into_moves {
    (|$from:ident|, $piece_bb:ident, |$to:ident|, $moves_bb:expr, $add_move:expr) => {{
        bitloop!(|$from|, $piece_bb, {
            let mut moves: Bitboard = $moves_bb;
            bitloop!(|$to|, moves, {
                $add_move
            });
        });
    }};
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MoveStage(u8);

impl MoveStage {
    #[rustfmt::skip]
    tuple_constants_enum!(Self,
        CAPTURE,
        QUIET
    );

    const fn new(data: u8) -> Self {
        Self(data)
    }

    fn increment(&mut self) {
        self.0 += 1;
    }
}

const MOVE_LIST_SIZE: usize = u8::MAX as usize;
struct MoveGenerator {
    stage: MoveStage,
    movelist: [Move; MOVE_LIST_SIZE],
    len: usize,
    index: usize,
}

impl MoveGenerator {
    const fn new() -> Self {
        Self {
            stage: MoveStage::new(0),
            movelist: [Move::nullmove(); MOVE_LIST_SIZE],
            len: 0,
            index: 0,
        }
    }

    fn add_move(&mut self, mv: Move) {
        self.movelist[self.len] = mv;
        self.len += 1;
    }

    fn gen_captures(&mut self, board: &Board) {
        let color = board.color_to_move;
        let them = board.them();

        let pawns = board.piece_bb(Piece::PAWN, color);
        let mut promoting_pawns = board.promotable_pawns();
        let mut normal_pawns = pawns.without(promoting_pawns);
        
        into_moves!(|from|, promoting_pawns, |to|, attacks::pawn(from, color).intersection(them), {
            self.add_move(Move::new_promo(to, from, Piece::QUEEN));
            self.add_move(Move::new_promo(to, from, Piece::KNIGHT));
            self.add_move(Move::new_promo(to, from, Piece::ROOK));
            self.add_move(Move::new_promo(to, from, Piece::BISHOP));
        });

        into_moves!(|from|, normal_pawns, |to|, attacks::pawn(from, color).intersection(them), self.add_move(Move::new_default(to, from)));

        
    }

    fn gen_quiets(&mut self, board: &Board) {
        todo!();
    }

    fn next(&mut self, board: &Board) {
        todo!();
    }
}
