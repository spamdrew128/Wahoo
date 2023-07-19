use engine::{
    board::board_representation::{Board, Piece, Square, NUM_RANKS, NUM_SQUARES},
    eval::evaluation::{
        phase, trace_of_position, EvalScore, Phase, EG, MG, NUM_PHASES, PHASES, PHASE_MAX,
    },
    eval::{piece_loop_eval::MoveCounts, trace::BackwardsPawns},
    eval::trace::{
        BishopPair, ForwardMobility, IsolatedPawns, MaterialPst, Mobility, Passer, PasserBlocker,
        PhalanxPawns, Safety, TempoBonus, Threats, TRACE_LEN,
    },
};
use std::{
    fs::{read_to_string, File},
    io::BufWriter,
    io::Write,
};

use crate::prev_weights::PREV_WEIGHTS;

const TUNER_VEC_LEN: usize = TRACE_LEN;
type TunerVec = [[f64; TUNER_VEC_LEN]; NUM_PHASES];

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
    fn new(board: &Board, game_result: f64) -> Self {
        let mut entry = Self {
            feature_vec: vec![],
            phase: phase(board),
            game_result,
        };

        let trace = trace_of_position(board);
        for (i, &value) in trace.iter().enumerate() {
            if value != 0 {
                entry.feature_vec.push(Feature::new(value, i));
            }
        }

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
    const CONVERGENCE_DELTA: f64 = 2e-7;
    const CONVERGENCE_CHECK_FREQ: u32 = 50;
    const MAX_EPOCHS: u32 = 20000;
    const LEARN_RATE: f64 = 0.12;

    fn new_weights(from_previous: bool) -> TunerVec {
        let mut result = [[0.0; TUNER_VEC_LEN]; NUM_PHASES];
        if from_previous {
            for phase in PHASES {
                for (i, &w) in PREV_WEIGHTS[phase].iter().enumerate() {
                    result[phase][i] = w;
                }
            }
        }

        result
    }

    pub fn new(from_previous: bool) -> Self {
        Self {
            entries: vec![],
            gradient: [[0.0; TUNER_VEC_LEN]; NUM_PHASES],
            weights: Self::new_weights(from_previous),
            momentum: [[0.0; TUNER_VEC_LEN]; NUM_PHASES],
            velocity: [[0.0; TUNER_VEC_LEN]; NUM_PHASES],
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
        self.gradient = [[0.0; TUNER_VEC_LEN]; NUM_PHASES];
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
                let grad_component: f64 = -2.0 * Self::K * self.gradient[phase][i] / (self.entries.len() as f64);
                self.momentum[phase][i] = BETA1 * self.momentum[phase][i] + (1.0 - BETA1) * grad_component;
                self.velocity[phase][i] = BETA2 * self.velocity[phase][i] + (1.0 - BETA2) * (grad_component * grad_component);

                self.weights[phase][i] -= (self.momentum[phase][i] / (EPSILON + self.velocity[phase][i].sqrt())) * Self::LEARN_RATE;
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
                self.create_weights_file();

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
        writeln!(
            output,
            "use crate::{{eval::{{evaluation::ScoreTuple, piece_tables::{{Pst, Prt}}}}, board::board_representation::NUM_PIECES}};\n"
        )
        .unwrap();

        writeln!(
            output,
            "const fn s(mg: i32, eg: i32) -> ScoreTuple {{ ScoreTuple::new(mg, eg) }}\n"
        )
        .unwrap();
    }

    fn write_pst<F>(&self, output: &mut BufWriter<File>, closing_str: &str, index_fn: F)
    where
        F: Fn(Square) -> usize,
    {
        write!(output, "Pst::new([").unwrap();
        for i in 0..NUM_SQUARES {
            let sq = Square::new(i);
            if i % 8 == 0 {
                write!(output, "\n  ").unwrap();
            }
            write!(
                output,
                "s({}, {}), ",
                self.weights[MG][index_fn(sq)] as EvalScore,
                self.weights[EG][index_fn(sq)] as EvalScore,
            )
            .unwrap();
        }
        writeln!(output, "\n]){closing_str}").unwrap();
    }

    fn write_prt<F>(&self, output: &mut BufWriter<File>, closing_str: &str, index_fn: F)
    where
        F: Fn(u8) -> usize,
    {
        write!(output, "Prt::new([").unwrap();
        for i in 0..NUM_RANKS {
            write!(
                output,
                "\n  s({}, {}),",
                self.weights[MG][index_fn(i)] as EvalScore,
                self.weights[EG][index_fn(i)] as EvalScore,
            )
            .unwrap();
        }
        writeln!(output, "\n]){closing_str}").unwrap();
    }

    fn write_material_psts(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = ["
        )
        .unwrap();

        for piece in Piece::LIST {
            writeln!(output, "// {} PST", piece.as_string().unwrap()).unwrap();
            self.write_pst(output, ",", |sq| MaterialPst::index(piece, sq));
        }

        writeln!(output, "];\n").unwrap();
    }

    fn write_passer_pst(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const PASSER_PST: Pst = ").unwrap();
        self.write_pst(output, ";\n", Passer::index);
    }

    fn write_passer_blocker_prt(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const PASSER_BLOCKERS_PRT: Prt = ").unwrap();
        self.write_prt(output, ";\n", PasserBlocker::index);
    }

    fn write_isolated_prt(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const ISOLATED_PAWNS_PRT: Prt = ").unwrap();
        self.write_prt(output, ";\n", IsolatedPawns::index);
    }

    fn write_phalanx_prt(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const PHALANX_PAWNS_PRT: Prt = ").unwrap();
        self.write_prt(output, ";\n", PhalanxPawns::index);
    }

    fn write_backwards_prt(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const BACKWARDS_PAWNS_PRT: Prt = ").unwrap();
        self.write_prt(output, ";\n", BackwardsPawns::index);
    }

    fn write_bishop_pair(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "pub const BISHOP_PAIR_BONUS: ScoreTuple = s({}, {});\n",
            self.weights[MG][BishopPair::index()] as EvalScore,
            self.weights[EG][BishopPair::index()] as EvalScore,
        )
        .unwrap();
    }

    fn write_mobility(&self, output: &mut BufWriter<File>) {
        for &piece in Piece::LIST.iter().take(4) {
            let init_line = format!(
                "pub const {}_MOBILITY: [ScoreTuple; {}] = [",
                piece.as_string().unwrap().to_uppercase(),
                Mobility::PIECE_MOVECOUNTS[piece.as_index()]
            );
            writeln!(output, "{}", init_line).unwrap();
            write!(output, "  ").unwrap();

            for i in 0..Mobility::PIECE_MOVECOUNTS[piece.as_index()] {
                let index = Mobility::index(piece, i);
                write!(
                    output,
                    "s({}, {}), ",
                    self.weights[MG][index] as EvalScore, self.weights[EG][index] as EvalScore,
                )
                .unwrap();
            }
            writeln!(output, "\n];\n").unwrap();
        }
    }

    fn write_forward_mobility(&self, output: &mut BufWriter<File>) {
        for &piece in Piece::LIST.iter().take(4) {
            let init_line = format!(
                "pub const {}_FORWARD_MOBILITY: [ScoreTuple; {}] = [",
                piece.as_string().unwrap().to_uppercase(),
                ForwardMobility::PIECE_MOVECOUNTS[piece.as_index()]
            );
            writeln!(output, "{}", init_line).unwrap();
            write!(output, "  ").unwrap();

            for i in 0..ForwardMobility::PIECE_MOVECOUNTS[piece.as_index()] {
                let index = ForwardMobility::index(piece, i);
                write!(
                    output,
                    "s({}, {}), ",
                    self.weights[MG][index] as EvalScore, self.weights[EG][index] as EvalScore,
                )
                .unwrap();
            }
            writeln!(output, "\n];\n").unwrap();
        }
    }

    fn write_safety(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "pub const KING_ZONE_ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = ["
        )
        .unwrap();
        for &piece in Piece::LIST.iter().take(5) {
            writeln!(output, "// {} attack values", piece.as_string().unwrap()).unwrap();
            write!(output, "[\n  ").unwrap();

            for i in 0..MoveCounts::QUEEN {
                let index = Safety::index(piece, i);
                write!(
                    output,
                    "s({}, {}), ",
                    self.weights[MG][index] as EvalScore, self.weights[EG][index] as EvalScore,
                )
                .unwrap();
            }
            writeln!(output, "\n],").unwrap();
        }
        writeln!(output, "];\n").unwrap();
    }

    fn write_threats(&self, output: &mut BufWriter<File>) {
        let strings = [
            "PAWN_THREAT_ON_KNIGHT",
            "PAWN_THREAT_ON_BISHOP",
            "PAWN_THREAT_ON_ROOK",
            "PAWN_THREAT_ON_QUEEN",
            "KNIGHT_THREAT_ON_BISHOP",
            "KNIGHT_THREAT_ON_ROOK",
            "KNIGHT_THREAT_ON_QUEEN",
            "BISHOP_THREAT_ON_KNIGHT",
            "BISHOP_THREAT_ON_ROOK",
            "BISHOP_THREAT_ON_QUEEN",
            "ROOK_THREAT_ON_QUEEN",
        ];

        for (i, s) in strings.iter().enumerate() {
            let index = Threats::START + i;
            writeln!(
                output,
                "pub const {}: ScoreTuple = s({}, {});",
                s, self.weights[MG][index] as EvalScore, self.weights[EG][index] as EvalScore,
            )
            .unwrap();
        }
    }

    fn write_tempo(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "\npub const TEMPO_BONUS: ScoreTuple = s({}, {});",
            self.weights[MG][TempoBonus::index()] as EvalScore,
            self.weights[EG][TempoBonus::index()] as EvalScore,
        )
        .unwrap();
    }

    fn create_output_file(&self) {
        let mut output = BufWriter::new(File::create("eval_constants.rs").unwrap());
        self.write_header(&mut output);
        self.write_material_psts(&mut output);
        self.write_passer_pst(&mut output);
        self.write_passer_blocker_prt(&mut output);
        self.write_isolated_prt(&mut output);
        self.write_phalanx_prt(&mut output);
        self.write_backwards_prt(&mut output);
        self.write_bishop_pair(&mut output);
        self.write_mobility(&mut output);
        self.write_forward_mobility(&mut output);
        self.write_safety(&mut output);
        self.write_threats(&mut output);
        self.write_tempo(&mut output);
    }

    fn create_weights_file(&self) {
        let mut output = BufWriter::new(File::create("prev_weights.rs").unwrap());
        writeln!(
            output,
            "#[rustfmt::skip]\npub const PREV_WEIGHTS: [[f64; {TUNER_VEC_LEN}]; {NUM_PHASES}] = [",
        )
        .unwrap();
        for phase in PHASES {
            writeln!(output, "[",).unwrap();
            for w in self.weights[phase] {
                write!(output, "{:.2}, ", w).unwrap();
            }
            writeln!(output, "],",).unwrap();
        }
        writeln!(output, "];",).unwrap();
    }
}
