use crate::chess_move::Move;
use crate::tuple_constants_enum;

#[derive(Debug, PartialEq, Eq)]
pub struct MoveStage(u8);

impl MoveStage {
    tuple_constants_enum!(Self,
        CAPTURES,
        QUIET
    );

    const fn new(data: u8) -> Self {
        Self(data)
    }
}

const MOVE_LIST_SIZE: usize = u8::MAX as usize;
struct MoveGenerator {
    movelist: [Move; MOVE_LIST_SIZE],
    len: usize,

    pub stage: MoveStage,
}

impl MoveGenerator {
    const fn new() -> Self {
        Self {
            movelist: [Move::nullmove(); MOVE_LIST_SIZE],
            len: 0,
            stage: MoveStage::new(0)
        }
    }
}
