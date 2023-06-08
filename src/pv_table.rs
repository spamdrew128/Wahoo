use crate::{
    chess_move::Move,
    search::{Ply, MAX_PLY},
};

struct PvTable {
    triangular_move_matrix: [[Move; MAX_PLY as usize]; MAX_PLY as usize],
    pv_length: [usize; (MAX_PLY + 1) as usize],
}

impl PvTable {
    const fn new() -> Self {
        Self {
            triangular_move_matrix: [[Move::nullmove(); MAX_PLY as usize]; MAX_PLY as usize],
            pv_length: [0; (MAX_PLY + 1) as usize],
        }
    }

    fn set_length(&mut self, ply: Ply) {
        self.pv_length[ply as usize] = ply as usize;
    }

    fn update(&mut self, ply: Ply, mv: Move) {
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

    fn best_move(&self) -> Move {
        let mv = self.triangular_move_matrix[0][0];
        assert!(mv != Move::nullmove());
        mv
    }
}
