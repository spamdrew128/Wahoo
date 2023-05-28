use crate::chess_move::Move;

const MOVE_LIST_SIZE: usize = u8::MAX as usize;
struct MoveGenerator {
    movelist: [Move; MOVE_LIST_SIZE],
    len: usize,
}

impl MoveGenerator {
    const fn new() -> Self {
        Self {
            movelist: [Move::nullmove(); MOVE_LIST_SIZE],
            len: 0
        }
    }
}
