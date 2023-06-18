use crate::{
    board_representation::{Board, NUM_COLORS, NUM_PIECES, NUM_SQUARES},
    chess_move::Move,
    search::Depth,
};

struct History {
    scores: [[[i16; NUM_SQUARES as usize]; NUM_PIECES as usize]; NUM_COLORS as usize],
}

impl History {
    const BONUS_MAX: i16 = 1200;
    const SCORE_MAX: i16 = i16::MAX;

    const fn new() -> Self {
        Self {
            scores: [[[0; NUM_SQUARES as usize]; NUM_PIECES as usize]; NUM_COLORS as usize],
        }
    }

    fn score(&self, board: &Board, mv: Move) -> i16 {
        let piece = board.piece_on_sq(mv.from()).as_index();
        let to = mv.to().as_index();
        let color = board.color_to_move.as_index();

        self.scores[color][piece][to]
    }

    fn update_history_score(&mut self, board: &Board, mv: Move, bonus: i16) {
        let scaled_bonus = bonus - self.score(board, mv) * bonus.abs() / Self::SCORE_MAX;

        let piece = board.piece_on_sq(mv.from()).as_index();
        let to = mv.to().as_index();
        let color = board.color_to_move.as_index();

        self.scores[color][piece][to] += scaled_bonus;
    }

    fn update(&mut self, board: &Board, quiets: &[Move], depth: Depth) {
        let d = i16::from(depth);
        let bonus = (16 * d * d).min(Self::BONUS_MAX);

        let cutoff_move = quiets[quiets.len() - 1];
        self.update_history_score(board, cutoff_move, bonus); // only the cutoff move gets a positive bonus
        for &mv in quiets.iter().take(quiets.len() - 1) {
            self.update_history_score(board, mv, -bonus);
        }
    }

    fn age_scores(&mut self) {
        self.scores
            .iter_mut()
            .flatten()
            .flatten()
            .for_each(|x| *x /= 2);
    }
}
