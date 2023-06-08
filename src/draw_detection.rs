use crate::{
    board_representation::Board,
    zobrist::{hash_position, ZobristHash},
};


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DrawDetector {
    zobrist_vec: Vec<ZobristHash>,
    halfmoves: u32,
}

impl DrawDetector {
    pub fn new(board: &Board) -> Self {
        Self {
            zobrist_vec: vec![hash_position(board)],
            halfmoves: 0,
        }
    }

    pub fn add_zobrist_hash(&mut self, hash: ZobristHash) {
        self.zobrist_vec.push(hash);
    }

    pub fn remove_zobrist_hash(&mut self) {
        self.zobrist_vec.pop();
    }

    pub fn current_zobrist_hash(&self) -> ZobristHash {
        let len = self.zobrist_vec.len();
        self.zobrist_vec[len - 1]
    }

    pub fn reset_halfmoves(&mut self) {
        self.halfmoves = 0;
    }

    pub fn increment_halfmoves(&mut self) {
        self.halfmoves += 1;
    }

    pub fn detected_draw(&self) -> bool {
        // twofold repetition check
        if self.zobrist_vec.len() < 4 {
            return false;
        }

        let current_hash = self.current_zobrist_hash();
        for &hash in self
            .zobrist_vec
            .iter()
            .rev()
            .take((self.halfmoves + 1) as usize)
            .skip(2)
            .step_by(2)
        {
            if hash == current_hash {
                return true;
            }
        }

        if self.halfmoves >= 100 {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn detects_draw() {
        use super::*;
        use crate::board_representation::START_FEN;
        use crate::{chess_move::Move, board_representation::Square};
        use crate::chess_move::Flag;

        let mut board = Board::from_fen(START_FEN);
        let mut detector = DrawDetector::new(&board);

        let w_knight_out = Move::new(Square::F3, Square::G1, Flag::NONE);
        let b_knight_out = Move::new(Square::F6, Square::G8, Flag::NONE);
        let w_knight_back = Move::new(Square::G1, Square::F3, Flag::NONE);
        let b_knight_back = Move::new(Square::G8, Square::F6, Flag::NONE);

        board.try_play_move(w_knight_out, &mut detector);
        board.try_play_move(b_knight_out, &mut detector);
        board.try_play_move(w_knight_back, &mut detector);

        assert!(!detector.detected_draw());

        board.try_play_move(b_knight_back, &mut detector);

        assert!(detector.detected_draw());
    }
}
