use crate::{
    board_representation::{Board, NUM_COLORS, NUM_PIECES, NUM_SQUARES},
    chess_move::Move,
};

struct History {
    scores: [[[i16; NUM_COLORS as usize]; NUM_PIECES as usize]; NUM_SQUARES as usize],
}

impl History {
    fn history_score(&self, board: &Board, mv: Move) -> i16 {
        let piece = board.piece_on_sq(mv.from()).as_index();
        let to = mv.to().as_index();
        let color = board.color_to_move.as_index();

        self.scores[color][piece][to]
    }
}
