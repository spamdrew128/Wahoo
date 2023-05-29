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
        START,
        CAPTURES,
        QUIETS
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
            stage: MoveStage::START,
            movelist: [Move::nullmove(); MOVE_LIST_SIZE],
            len: 0,
            index: 0,
        }
    }

    const fn stage_complete(&self) -> bool {
        self.index >= self.len
    }
    
    fn advance_stage(&mut self) {
        self.stage.increment();
        self.len = 0;
        self.index = 0;
    }

    fn add_move(&mut self, mv: Move) {
        self.movelist[self.len] = mv;
        self.len += 1;
    }

    fn next_move_in_stage(&mut self) -> Move {
        let mv = self.movelist[self.index];
        self.index += 1;
        mv
    }

    fn gen_captures(&mut self, board: &Board) {
        let color = board.color_to_move;
        let them = board.them();
        let occupied = board.occupied();

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

        if let Some(to) = board.ep_sq {
            let mut attackers = attacks::pawn(to, color.flip()).intersection(pawns);
            bitloop!(|from|, attackers, {
                self.add_move(Move::new_ep(to, from));
            });
        }

        let mut knights = board.piece_bb(Piece::KNIGHT, color);
        into_moves!(|from|, knights, |to|, attacks::knight(from).intersection(them), self.add_move(Move::new_default(to, from)));

        let mut bishops = board.piece_bb(Piece::BISHOP, color);
        into_moves!(|from|, bishops, |to|, attacks::bishop(from, occupied).intersection(them), self.add_move(Move::new_default(to, from)));

        let mut rooks = board.piece_bb(Piece::ROOK, color);
        into_moves!(|from|, rooks, |to|, attacks::rook(from, occupied).intersection(them), self.add_move(Move::new_default(to, from)));

        let mut queens = board.piece_bb(Piece::QUEEN, color);
        into_moves!(|from|, queens, |to|, attacks::queen(from, occupied).intersection(them), self.add_move(Move::new_default(to, from)));

        let mut king = board.piece_bb(Piece::KING, color);
        into_moves!(|from|, king, |to|, attacks::king(from).intersection(them), self.add_move(Move::new_default(to, from)));
    }

    fn gen_quiets(&mut self, board: &Board) {
        todo!();
    }

    fn next(&mut self, board: &Board) -> Option<Move> {
        while self.stage_complete() {
            self.advance_stage();

            match self.stage {
                MoveStage::CAPTURES => self.gen_captures(board),
                MoveStage::QUIETS => (),
                _ => return None,
            }
        }

        Some(self.next_move_in_stage())
    }
}

#[cfg(test)]
mod tests {
    use crate::board_representation::NUM_PIECES;

    #[test]
    fn generates_captures() {
        use super::*;

        let board = Board::from_fen("1n4K1/P2k2b1/4r1n1/PpPB4/5N2/bRq1r3/3P4/2Q5 w - b6 0 2");
        let mut counts = [0; NUM_PIECES as usize];
        let mut promo_count = 0;
        let mut ep_count = 0;

        let mut generator = MoveGenerator::new();
        while let Some(mv) = generator.next(&board) {
            let piece = board.piece_on_sq(mv.from());
            counts[piece.as_index()] += 1;

            if mv.is_promo() {
                promo_count += 1;
            }  

            if mv.is_ep() {
                ep_count += 1;
            }
        }

        assert_eq!(counts[Piece::PAWN.as_index()], 8);
        assert_eq!(counts[Piece::BISHOP.as_index()], 1);
        assert_eq!(counts[Piece::ROOK.as_index()], 3);
        assert_eq!(counts[Piece::QUEEN.as_index()], 2);
        assert_eq!(counts[Piece::KNIGHT.as_index()], 2);
        assert_eq!(counts[Piece::KING.as_index()], 1);
        assert_eq!(promo_count, 4);
        assert_eq!(ep_count, 2);
    }
}