use crate::{
    chess_move::Move,
    search::{Ply, MAX_PLY},
};

#[derive(Debug)]
pub struct PvTable {
    triangular_move_matrix: [[Move; MAX_PLY as usize]; MAX_PLY as usize],
    pv_length: [usize; (MAX_PLY + 1) as usize],
}

impl PvTable {
    pub const fn new() -> Self {
        Self {
            triangular_move_matrix: [[Move::nullmove(); MAX_PLY as usize]; MAX_PLY as usize],
            pv_length: [0; (MAX_PLY + 1) as usize],
        }
    }

    pub fn set_length(&mut self, ply: Ply) {
        self.pv_length[ply as usize] = ply as usize;
    }

    pub fn update(&mut self, ply: Ply, mv: Move) {
        let ply = ply as usize;

        let new_len = self.pv_length[ply + 1];
        self.pv_length[ply] = new_len;
        self.triangular_move_matrix[ply][ply] = mv;

        let copy_start = ply + 1;
        let copy_end = new_len;
        for i in copy_start..copy_end {
            self.triangular_move_matrix[ply][i] = self.triangular_move_matrix[ply + 1][i];
        }
    }

    pub fn best_move(&self) -> Move {
        let mv = self.triangular_move_matrix[0][0];
        assert!(mv != Move::nullmove());
        mv
    }

    pub fn pv_string(&self) -> String {
        let mut result = String::new();
        let variation_length = self.pv_length[0];
        let pv = &self.triangular_move_matrix[0][..variation_length];

        for mv in pv {
            result.push(' ');
            result.push_str(mv.as_string().as_str());
        }
        result
    }
}
