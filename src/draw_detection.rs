use crate::{zobrist::{ZobristHash, hash_position}, board_representation::Board};

struct DrawDetector {
    zobrist_vec: Vec<ZobristHash>,
    halfmoves: u32,
}

impl DrawDetector {
    pub fn new(board: &Board) -> Self {
        Self { zobrist_vec: vec![hash_position(board)], halfmoves: 0 }
    }

    pub fn add_zobrist_hash(&mut self, hash: ZobristHash) {
        self.zobrist_vec.push(hash);
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

    pub fn position_is_drawn(&self) -> bool {
        // twofold repetition check
        if self.zobrist_vec.len() < 4 {
            return false;
        }

        let current_hash = self.current_zobrist_hash();
        for &hash in self.zobrist_vec.iter().rev().take(self.halfmoves as usize).skip(2).step_by(2) {
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
