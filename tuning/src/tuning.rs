use engine::board_representation::{NUM_SQUARES, NUM_PIECES};

const MG: usize = 0;
const EG: usize = 1;
const NUM_PHASES: usize = 2;

struct Pst;
impl Pst {
    const START: usize = 0;
    const LEN: usize = (NUM_PIECES as usize) * (NUM_SQUARES as usize);
}

struct Gradient {
    linear: [[f64; Pst::LEN]; NUM_PHASES]
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
