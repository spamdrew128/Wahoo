use crate::board_representation::{Bitboard, Board};
use crate::chess_move::Move;
use crate::tuple_constants_enum;

#[derive(Debug, PartialEq, Eq)]
pub struct MoveStage(u8);

impl MoveStage {
    tuple_constants_enum!(Self, CAPTURE, QUIET);

    const fn new(data: u8) -> Self {
        Self(data)
    }

    fn increment(&mut self) {
        self.0 += 1;
    }
}

const MOVE_LIST_SIZE: usize = u8::MAX as usize;
struct MoveGenerator {
    pub stage: MoveStage,

    movelist: [Move; MOVE_LIST_SIZE],
    len: usize,
    index: usize,
}

impl MoveGenerator {
    const fn new(board: &Board) -> Self {
        Self {
            stage: MoveStage::new(0),
            movelist: [Move::nullmove(); MOVE_LIST_SIZE],
            len: 0,
            index: 0,
        }
    }

    fn next(&mut self, board: &Board) {
        todo!();
    }

    fn gen_captures(&mut self, board: &Board) {
        todo!();
    }

    fn gen_quiets(&mut self, board: &Board) {
        todo!();
    }
}
