use crate::{
    board_representation::Board,
    zobrist::{hash_position, ZobristHash},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ZobristStack {
    zobrist_vec: Vec<ZobristHash>,
}

impl ZobristStack {
    pub fn new(board: &Board) -> Self {
        Self {
            zobrist_vec: vec![hash_position(board)],
        }
    }

    pub fn add_hash(&mut self, hash: ZobristHash) {
        self.zobrist_vec.push(hash);
    }

    pub fn revert_state(&mut self) {
        self.zobrist_vec.pop();
    }

    pub fn current_zobrist_hash(&self) -> ZobristHash {
        let len = self.zobrist_vec.len();
        self.zobrist_vec[len - 1]
    }

    pub fn twofold_repetition(&self, halfmoves: u16) -> bool {
        if self.zobrist_vec.len() < 4 {
            return false;
        }

        let current_hash = self.current_zobrist_hash();
        for &hash in self
            .zobrist_vec
            .iter()
            .rev()
            .take((halfmoves + 1) as usize)
            .skip(2)
            .step_by(2)
        {
            if hash == current_hash {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn twofold_repetition_works() {
        use super::*;
        use crate::board_representation::START_FEN;
        use crate::chess_move::Flag;
        use crate::{board_representation::Square, chess_move::Move};

        let mut board = Board::from_fen(START_FEN);
        let mut detector = ZobristStack::new(&board);

        let w_knight_out = Move::new(Square::F3, Square::G1, Flag::NONE);
        let b_knight_out = Move::new(Square::F6, Square::G8, Flag::NONE);
        let w_knight_back = Move::new(Square::G1, Square::F3, Flag::NONE);
        let b_knight_back = Move::new(Square::G8, Square::F6, Flag::NONE);

        board.try_play_move(w_knight_out, &mut detector);
        board.try_play_move(b_knight_out, &mut detector);
        board.try_play_move(w_knight_back, &mut detector);

        assert!(!detector.twofold_repetition(board.halfmoves));

        board.try_play_move(b_knight_back, &mut detector);

        assert!(detector.twofold_repetition(board.halfmoves));
    }
}
