use engine::board_representation::{Piece, Square, NUM_PIECES, NUM_SQUARES};

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
    linear: [[f64; Pst::LEN]; NUM_PHASES],
}

impl Gradient {
    const fn new() -> Self {
        Self {
            linear: [[0.0; Pst::LEN]; NUM_PHASES],
        }
    }
}

struct Feature {
    data: i8,
    index: usize,
}

struct Entry {
    feature_vec: Vec<Feature>,
    phase: u8,
    result: i8,
}

struct Tuner {
    entries: Vec<Entry>,
    gradient: Gradient,
}

impl Tuner {
    fn new() -> Self {
        Self {
            entries: vec![],
            gradient: Gradient::new(),
        }
    }
}
