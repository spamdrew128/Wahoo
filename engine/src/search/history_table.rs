use crate::{
    board::board_representation::{NUM_COLORS, NUM_PIECES, NUM_SQUARES},
    board::{chess_move::Move, board_representation::Color},
    eval::evaluation::EvalScore,
    search::search::Depth,
};

#[derive(Debug, Clone)]
pub struct History {
    scores: [[[EvalScore; NUM_SQUARES as usize]; NUM_PIECES as usize]; NUM_COLORS as usize],
}

impl History {
    const BONUS_MAX: i32 = 1200;
    const SCORE_MAX: i32 = i16::MAX as i32;

    pub const fn new() -> Self {
        Self {
            scores: [[[0; NUM_SQUARES as usize]; NUM_PIECES as usize]; NUM_COLORS as usize],
        }
    }

    pub const fn score(&self, color: Color, mv: Move) -> EvalScore {
        let piece = mv.piece().as_index();
        let to = mv.to().as_index();

        self.scores[color.as_index()][piece][to]
    }

    fn update_history_score(&mut self, color: Color, mv: Move, bonus: i32) {
        let scaled_bonus = bonus - self.score(color, mv) * bonus.abs() / Self::SCORE_MAX;

        let piece = mv.piece().as_index();
        let to = mv.to().as_index();

        self.scores[color.as_index()][piece][to] += scaled_bonus;
    }

    pub fn update(&mut self, color: Color, quiets: &[Move], depth: Depth) {
        let d = i32::from(depth);
        let bonus = (16 * d * d).min(Self::BONUS_MAX);

        let cutoff_move = quiets[quiets.len() - 1];
        self.update_history_score(color, cutoff_move, bonus); // only the cutoff move gets a positive bonus
        for &mv in quiets.iter().take(quiets.len() - 1) {
            self.update_history_score(color, mv, -bonus);
        }
    }

    pub fn age_scores(&mut self) {
        self.scores
            .iter_mut()
            .flatten()
            .flatten()
            .for_each(|x| *x /= 2);
    }
}
