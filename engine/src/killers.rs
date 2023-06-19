use crate::{
    chess_move::Move,
    search::{Ply, MAX_PLY},
};

struct Killers {
    moves: [Move; MAX_PLY as usize],
}

impl Killers {
    pub const fn new() -> Self {
        Self {
            moves: [Move::nullmove(); MAX_PLY as usize],
        }
    }

    pub fn update(&mut self, ply: Ply, mv: Move) {
        self.moves[ply as usize] = mv;
    }

    pub const fn killer(&self, ply: Ply) -> Move {
        self.moves[ply as usize]
    }
}
