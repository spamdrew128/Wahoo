use engine::{
    board_representation::{Board, Color, Piece, Square, NUM_PIECES, NUM_SQUARES},
    evaluation::{phase, EvalScore, Phase, EG, MG, NUM_PHASES, PHASES, PHASE_MAX},
};
use std::{
    fs::{read_to_string, File},
    io::BufWriter,
    io::Write,
};

type TunerVec = [[f64; Pst::LEN]; NUM_PHASES];

struct Pst;
impl Pst {
    const START: usize = 0;
    const LEN: usize = (NUM_PIECES as usize) * (NUM_SQUARES as usize);

    fn index(piece: Piece, sq: Square) -> usize {
        Self::START + usize::from(NUM_SQUARES) * piece.as_index() + sq.as_index()
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
    feature_vec: Vec<Feature>,
    phase: Phase,
    game_result: f64,
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

                let value = (w_sq.as_bitboard().intersection(w_piece_bb).popcount() as i8)
                    - (b_sq.as_bitboard().intersection(b_piece_bb).popcount() as i8);
                if value != 0 {
                    self.feature_vec
                        .push(Feature::new(value, Pst::index(piece, sq)));
                }
            }
        }
    }

    fn new(board: &Board, game_result: f64) -> Self {
        let mut entry = Self {
            feature_vec: vec![],
            phase: phase(board),
            game_result,
        };

        entry.add_pst_features(board);

        entry
    }

    fn evaluation(&self, weights: &TunerVec) -> f64 {
        let mut scores = [0.0, 0.0];

        for phase in PHASES {
            for feature in &self.feature_vec {
                scores[phase] += f64::from(feature.value) * weights[phase][feature.index];
            }
        }

        (scores[MG] * self.mg_phase() + scores[EG] * self.eg_phase()) / f64::from(PHASE_MAX)
    }

    fn mg_phase(&self) -> f64 {
        f64::from(self.phase)
    }

    fn eg_phase(&self) -> f64 {
        f64::from(PHASE_MAX - self.phase)
    }
}

pub struct Tuner {
    entries: Vec<Entry>,
    gradient: TunerVec,
    weights: TunerVec,
    momentum: TunerVec,
    velocity: TunerVec,
}

impl Tuner {
    const K: f64 = 0.006634;
    const CONVERGENCE_DELTA: f64 = 1e-9;
    const CONVERGENCE_CHECK_FREQ: u32 = 50;
    const MAX_EPOCHS: u32 = 20000;

    fn new_weights(from_zero: bool) -> TunerVec {
        if from_zero {
            return [[0.0; Pst::LEN]; NUM_PHASES];
        }

        let scores: [EvalScore; NUM_PIECES as usize] = [300, 320, 500, 900, 100, 0];
        let mut result = [[0.0; Pst::LEN]; NUM_PHASES];

        for piece in Piece::LIST {
            for i in 0..NUM_SQUARES {
                let index = Pst::index(piece, Square::new(i));
                result[MG][index] = scores[piece.as_index()] as f64;
                result[EG][index] = scores[piece.as_index()] as f64;
            }
        }
        result
    }

    pub fn new(from_zero: bool) -> Self {
        Self {
            entries: vec![],
            gradient: [[0.0; Pst::LEN]; NUM_PHASES],
            weights: Self::new_weights(from_zero),
            momentum: [[0.0; Pst::LEN]; NUM_PHASES],
            velocity: [[0.0; Pst::LEN]; NUM_PHASES],
        }
    }

    pub fn load_from_file(&mut self, file_name: &str) {
        for line in read_to_string(file_name).unwrap().lines() {
            let (fen, r) = line.split_once('[').unwrap();
            let game_result = r.split_once(']').unwrap().0.parse::<f64>().unwrap();

            let board = Board::from_fen(fen);
            self.entries.push(Entry::new(&board, game_result));
        }
        println!("Loaded file: begin tuning...\n");
    }

    pub fn reset_gradient(&mut self) {
        self.gradient = [[0.0; Pst::LEN]; NUM_PHASES];
    }

    fn sigmoid(e: f64) -> f64 {
        1.0 / (1.0 + (f64::exp(-Self::K * e)))
    }

    fn sigmoid_prime(sigmoid: f64) -> f64 {
        // K is omitted for now but will be added later
        sigmoid * (1.0 - sigmoid)
    }

    fn update_entry_gradient_component(entry: &Entry, gradient: &mut TunerVec, weights: &TunerVec) {
        let r = entry.game_result;
        let eval = entry.evaluation(weights);
        let sigmoid = Self::sigmoid(eval);
        let sigmoid_prime = Self::sigmoid_prime(sigmoid);

        let coeffs: [f64; NUM_PHASES] = [
            ((r - sigmoid) * sigmoid_prime * entry.mg_phase()) / f64::from(PHASE_MAX),
            ((r - sigmoid) * sigmoid_prime * entry.eg_phase()) / f64::from(PHASE_MAX),
        ];

        for phase in PHASES {
            for feature in &entry.feature_vec {
                gradient[phase][feature.index] += coeffs[phase] * f64::from(feature.value);
            }
        }
    }

    fn update_gradient(&mut self) {
        for entry in &self.entries {
            Self::update_entry_gradient_component(entry, &mut self.gradient, &self.weights);
        }
    }

    #[rustfmt::skip]
    fn update_weights(&mut self) {
        const BETA1: f64 = 0.9;
        const BETA2: f64 = 0.999;
        const EPSILON: f64 = 1e-8;

        for i in 0..self.gradient[0].len() {
            for phase in PHASES {
                // we left off k eariler, so we add it back here
                let grad_component: f64 = -Self::K * self.gradient[phase][i] / (self.entries.len() as f64);
                self.momentum[phase][i] = BETA1 * self.momentum[phase][i] + (1.0 - BETA1) * grad_component;
                self.velocity[phase][i] = BETA2 * self.velocity[phase][i] + (1.0 - BETA2) * (grad_component * grad_component);

                self.weights[phase][i] -= self.momentum[phase][i] / (EPSILON + self.velocity[phase][i].sqrt());
            }
        }
    }

    fn mse(&self) -> f64 {
        let mut total_error = 0.0;
        for entry in &self.entries {
            let eval = entry.evaluation(&self.weights);
            let error = entry.game_result - Self::sigmoid(eval);
            total_error += error * error;
        }

        total_error / (self.entries.len() as f64)
    }

    pub fn train(&mut self) {
        let mut prev_mse = self.mse();
        for epoch in 0..Self::MAX_EPOCHS {
            self.reset_gradient();
            self.update_gradient();
            self.update_weights();

            if epoch % Self::CONVERGENCE_CHECK_FREQ == 0 {
                let mse = self.mse();
                let delta_mse = prev_mse - mse;
                println!("Epoch: {epoch}");
                println!("MSE: {mse}");
                println!("MSE change since previous: {delta_mse}\n");

                self.create_output_file();

                if delta_mse < Self::CONVERGENCE_DELTA {
                    return;
                }
                prev_mse = mse;
            }
        }
    }

    #[allow(clippy::write_literal)]
    fn write_header(&self, output: &mut BufWriter<File>) {
        writeln!(output, "#![cfg_attr(rustfmt, rustfmt_skip)]").unwrap();
        writeln!(output, "use crate::{{evaluation::ScoreTuple, board_representation::{{NUM_SQUARES, NUM_PIECES}}}};\n").unwrap();

        writeln!(
            output,
            "{}\n{}\n{}\n{}\n{}\n",
            "macro_rules! s {",
            "  ($mg:expr, $eg:expr) => {",
            "    ScoreTuple::new($mg, $eg)",
            "  };",
            "}",
        )
        .unwrap();
    }

    fn write_psts(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "pub const PST: [[ScoreTuple; NUM_SQUARES as usize]; NUM_PIECES as usize] = ["
        )
        .unwrap();

        for piece in Piece::LIST {
            writeln!(output, "// {} PST", piece.as_string().unwrap()).unwrap();
            write!(output, "[").unwrap();
            for i in 0..NUM_SQUARES {
                let sq = Square::new(i);
                if i % 8 == 0 {
                    write!(output, "\n  ").unwrap();
                }
                write!(
                    output,
                    "s!({}, {}), ",
                    self.weights[MG][Pst::index(piece, sq)] as EvalScore,
                    self.weights[EG][Pst::index(piece, sq)] as EvalScore,
                )
                .unwrap();
            }
            writeln!(output, "\n],").unwrap();
        }

        writeln!(output, "];").unwrap();
    }

    fn create_output_file(&self) {
        let mut output = BufWriter::new(File::create("eval_constants.rs").unwrap());
        self.write_header(&mut output);
        self.write_psts(&mut output);
    }
}
