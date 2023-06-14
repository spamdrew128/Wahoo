use engine::{
    board_representation::{Board, Color, Piece, Square, NUM_PIECES, NUM_SQUARES},
    evaluation::{phase, Phase},
};
use std::{fs::read_to_string, result};

const MG: usize = 0;
const EG: usize = 1;
const NUM_PHASES: usize = 2;

struct Pst;
impl Pst {
    const START: usize = 0;
    const LEN: usize = (NUM_PIECES as usize) * (NUM_SQUARES as usize);

    fn index(piece: Piece, sq: Square) -> usize {
        usize::from(NUM_SQUARES) * piece.as_index() + sq.as_index()
    }
}

struct Gradient {
    data: [[f64; Pst::LEN]; NUM_PHASES], // add sections for "linear" and "non-linear" later
}

impl Gradient {
    const fn new() -> Self {
        Self {
            data: [[0.0; Pst::LEN]; NUM_PHASES],
        }
    }
}

struct Weights {
    data: [[f64; Pst::LEN]; NUM_PHASES], // add sections for "linear" and "non-linear" later
}

impl Weights {
    const fn new() -> Self {
        Self {
            data: [[0.0; Pst::LEN]; NUM_PHASES],
        }
    }
}

struct Feature {
    value: i8,
    index: usize,
}

impl Feature {
    fn new(value: i8, index: usize) -> Self {
        Self { value, index }
    }
}

struct Entry {
    linear_feature_vec: Vec<Feature>,
    phase: Phase,
    game_result: i8,
}

impl Entry {
    fn add_pst_features(&mut self, board: &Board) {
        for piece in Piece::LIST {
            let w_piece_bb = board.piece_bb(piece, Color::White);
            let b_piece_bb = board.piece_bb(piece, Color::Black);
            for i in 0..NUM_SQUARES {
                let sq = Square::new(i);
                let w_sq = sq.flip();
                let b_sq = sq;

                let value = w_sq.as_bitboard().intersection(w_piece_bb).popcount()
                    - b_sq.as_bitboard().intersection(b_piece_bb).popcount();
                if value != 0 {
                    self.linear_feature_vec
                        .push(Feature::new(value as i8, Pst::index(piece, sq)));
                }
            }
        }
    }

    fn new(board: &Board, game_result: i8) -> Self {
        let mut entry = Self {
            linear_feature_vec: vec![],
            phase: phase(board),
            game_result,
        };

        entry.add_pst_features(board);

        entry
    }
}

pub struct Tuner {
    entries: Vec<Entry>,
    gradient: Gradient,
    weights: Weights,
}

impl Tuner {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            gradient: Gradient::new(),
            weights: Weights::new(),
        }
    }

    pub fn load_from_file(&mut self, file_name: &str) {
        for line in read_to_string(file_name).unwrap().lines() {
            let (fen, r) = line.split_once('[').unwrap();
            let game_result = r.split_once(']').unwrap().0.parse::<i8>().unwrap();

            let board = Board::from_fen(fen);
            self.entries.push(Entry::new(&board, game_result));
        }
    }
}
