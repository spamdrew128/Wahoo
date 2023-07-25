use crate::tuner_val::S;
use engine::{
    board::board_representation::{
        Board, Color, Piece, Square, NUM_COLORS, NUM_RANKS, NUM_SQUARES,
    },
    eval::evaluation::{phase, trace_of_position, Phase, PHASE_MAX},
    eval::trace::{
        BishopPair, EnemyKingRank, ForwardMobility, IsolatedPawns, MaterialPst, Mobility, Passer,
        PasserBlocker, PhalanxPawns, TempoBonus, Threats, LINEAR_TRACE_LEN,
    },
    eval::{
        evaluation::SAFETY_LIMIT,
        trace::{Attacks, Defenses, EnemyVirtMobility, SAFETY_TRACE_LEN},
    },
};
use std::{
    fs::{read_to_string, File},
    io::BufWriter,
    io::Write,
    thread,
};

struct TunerStruct {
    linear: [S; LINEAR_TRACE_LEN],
    safety: [S; SAFETY_TRACE_LEN],
}

impl TunerStruct {
    fn new() -> Self {
        Self {
            linear: [S::new(0.0, 0.0); LINEAR_TRACE_LEN],
            safety: [S::new(0.0, 0.0); SAFETY_TRACE_LEN],
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
    feature_vec: Vec<Feature>,
    safety_feature_vec: [Vec<Feature>; NUM_COLORS as usize],
    phase: Phase,
    game_result: f64,
}

impl Entry {
    fn new(board: &Board, game_result: f64) -> Self {
        let mut entry = Self {
            feature_vec: vec![],
            safety_feature_vec: [vec![], vec![]],
            phase: phase(board),
            game_result,
        };

        let trace = trace_of_position(board);
        for (i, &value) in trace.linear.iter().enumerate() {
            if value != 0 {
                entry.feature_vec.push(Feature::new(value, i));
            }
        }

        for color in Color::LIST {
            for (i, &value) in trace.safety[color.as_index()].iter().enumerate() {
                if value != 0 {
                    entry.safety_feature_vec[color.as_index()].push(Feature::new(value, i));
                }
            }
        }

        entry
    }

    fn inner_safety_score(&self, weights: &TunerStruct) -> (S, S) {
        let mut attack_power = [S::new(0.0, 0.0), S::new(0.0, 0.0)];
        for color in Color::LIST {
            for feature in &self.safety_feature_vec[color.as_index()] {
                attack_power[color.as_index()] +=
                    f64::from(feature.value) * weights.safety[feature.index];
            }
        }

        (
            attack_power[Color::White.as_index()],
            attack_power[Color::Black.as_index()],
        )
    }

    fn evaluation(&self, weights: &TunerStruct) -> f64 {
        let mut score = S::new(0.0, 0.0);

        for feature in &self.feature_vec {
            score += f64::from(feature.value) * weights.linear[feature.index];
        }

        let (w_ap, b_ap) = self.inner_safety_score(weights);
        let limit = f64::from(SAFETY_LIMIT);
        score +=
            (0.01 * w_ap.max(0.0).square()).min(limit) - (0.01 * b_ap.max(0.0).square()).min(limit);

        (score.mg() * self.mg_phase() + score.eg() * self.eg_phase()) / f64::from(PHASE_MAX)
    }

    fn error(&self, weights: &TunerStruct) -> f64 {
        let eval = self.evaluation(weights);
        let error = self.game_result - Tuner::sigmoid(eval);
        error * error
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
    gradient: TunerStruct,
    weights: TunerStruct,
    momentum: TunerStruct,
    velocity: TunerStruct,
    threads: usize,
}

macro_rules! update_weights {
    ($self:ident, $type:ident) => {{
        const BETA1: f64 = 0.9;
        const BETA2: f64 = 0.999;
        const EPSILON: f64 = 1e-8;

        for i in 0..$self.gradient.$type.len() {
            // we left off k eariler, so we add it back here
            let grad_component: S =
                -2.0 * Tuner::K * $self.gradient.$type[i] / ($self.entries.len() as f64);

            $self.momentum.$type[i] =
                BETA1 * $self.momentum.$type[i] + (1.0 - BETA1) * grad_component;
            $self.velocity.$type[i] =
                BETA2 * $self.velocity.$type[i] + (1.0 - BETA2) * (grad_component * grad_component);

            $self.weights.$type[i] -= ($self.momentum.$type[i]
                / (EPSILON + $self.velocity.$type[i].sqrt()))
                * Self::LEARN_RATE;
        }
    }};
}

impl Tuner {
    const K: f64 = 0.006634;
    const CONVERGENCE_DELTA: f64 = 7e-7;
    const CONVERGENCE_CHECK_FREQ: u32 = 50;
    const MAX_EPOCHS: u32 = 20000;
    const LEARN_RATE: f64 = 0.12;

    fn new_weights() -> TunerStruct {
        let mut result = TunerStruct::new();
        let vals = [300.0, 300.0, 500.0, 900.0, 100.0, 0.0];
        for piece in Piece::LIST {
            let w = vals[piece.as_index()];
            for sq in 0..NUM_SQUARES {
                result.linear[MaterialPst::index(piece, Square::new(sq))] = S::new(w, w);
            }
        }

        for w in result.safety.iter_mut() {
            *w = S::new(1.0, 1.0);
        }

        result
    }

    pub fn new(threads: usize) -> Self {
        Self {
            entries: vec![],
            gradient: TunerStruct::new(),
            weights: Self::new_weights(),
            momentum: TunerStruct::new(),
            velocity: TunerStruct::new(),
            threads,
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
        self.gradient = TunerStruct::new();
    }

    fn sigmoid(e: f64) -> f64 {
        1.0 / (1.0 + (f64::exp(-Self::K * e)))
    }

    fn sigmoid_prime(sigmoid: f64) -> f64 {
        // K is omitted for now but will be added later
        sigmoid * (1.0 - sigmoid)
    }

    fn safety_prime(x: f64) -> f64 {
        if x > 0.0 && x < 10.0 * f64::from(SAFETY_LIMIT).sqrt() {
            0.01 * 2.0 * x
        } else {
            0.0
        }
    }

    fn update_entry_gradient_component(
        entry: &Entry,
        gradient: &mut TunerStruct,
        weights: &TunerStruct,
    ) {
        let r = entry.game_result;
        let eval = entry.evaluation(weights);
        let sigmoid = Self::sigmoid(eval);
        let sigmoid_prime = Self::sigmoid_prime(sigmoid);

        let coeff = S::new(
            ((r - sigmoid) * sigmoid_prime * entry.mg_phase()) / f64::from(PHASE_MAX),
            ((r - sigmoid) * sigmoid_prime * entry.eg_phase()) / f64::from(PHASE_MAX),
        );

        for feature in &entry.feature_vec {
            gradient.linear[feature.index] += coeff * f64::from(feature.value);
        }

        let (x_w, x_b) = entry.inner_safety_score(weights);
        let (x_w_prime, x_b_prime) = (
            S::new(Self::safety_prime(x_w.mg()), Self::safety_prime(x_w.eg())),
            S::new(Self::safety_prime(x_b.mg()), Self::safety_prime(x_b.eg())),
        );
        for color in Color::LIST {
            let x_prime = if color == Color::White {
                x_w_prime
            } else {
                -x_b_prime
            };

            for feature in &entry.safety_feature_vec[color.as_index()] {
                gradient.safety[feature.index] += coeff * x_prime * f64::from(feature.value);
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
        update_weights!(self, linear);
        update_weights!(self, safety);
    }

    fn mse(&self) -> f64 {
        let size = self.entries.len() / self.threads;
        thread::scope(|s| {
            self.entries
                .chunks(size)
                .map(|chunk| {
                    s.spawn(|| {
                        chunk
                            .iter()
                            .map(|entry| entry.error(&self.weights))
                            .sum::<f64>()
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|p| p.join().unwrap_or_default())
                .sum::<f64>()
        }) / (self.entries.len() as f64)
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
                // self.create_weights_file();

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
            let w = self.weights.linear[index_fn(sq)];
            write!(output, "{w}, ",).unwrap();
        }
        writeln!(output, "\n]){closing_str}").unwrap();
    }

    fn write_prt<F>(&self, output: &mut BufWriter<File>, closing_str: &str, vals: &[S], index_fn: F)
    where
        F: Fn(u8) -> usize,
    {
        write!(output, "Prt::new([").unwrap();
        for i in 0..NUM_RANKS {
            let w = vals[index_fn(i)];
            write!(output, "\n  {w},").unwrap();
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
        self.write_prt(
            output,
            ";\n",
            self.weights.linear.as_slice(),
            PasserBlocker::index,
        );
    }

    fn write_isolated_prt(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const ISOLATED_PAWNS_PRT: Prt = ").unwrap();
        self.write_prt(
            output,
            ";\n",
            self.weights.linear.as_slice(),
            IsolatedPawns::index,
        );
    }

    fn write_phalanx_prt(&self, output: &mut BufWriter<File>) {
        write!(output, "pub const PHALANX_PAWNS_PRT: Prt = ").unwrap();
        self.write_prt(
            output,
            ";\n",
            self.weights.linear.as_slice(),
            PhalanxPawns::index,
        );
    }

    fn write_bishop_pair(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "pub const BISHOP_PAIR_BONUS: ScoreTuple = {};\n",
            self.weights.linear[BishopPair::index()]
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
                let w = self.weights.linear[index];
                write!(output, "{w}, ",).unwrap();
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
                let w = self.weights.linear[index];
                write!(output, "{w}, ",).unwrap();
            }
            writeln!(output, "\n];\n").unwrap();
        }
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
            let w = self.weights.linear[index];
            writeln!(output, "pub const {s}: ScoreTuple = {w};",).unwrap();
        }
    }

    fn write_tempo(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "\npub const TEMPO_BONUS: ScoreTuple = {};",
            self.weights.linear[TempoBonus::index()]
        )
        .unwrap();
    }

    fn write_safety(&self, output: &mut BufWriter<File>) {
        write!(
            output,
            "\npub const ENEMY_VIRT_MOBILITY: [ScoreTuple; 28] = [\n  ",
        )
        .unwrap();
        for i in 0..EnemyVirtMobility::LEN {
            let index = EnemyVirtMobility::index(i);
            let w = self.weights.safety[index];
            write!(output, "{w}, ").unwrap();
        }
        writeln!(output, "\n];",).unwrap();

        write!(
            output,
            "\npub const ATTACKS: [ScoreTuple; (NUM_PIECES - 1) as usize] = [\n  ",
        )
        .unwrap();
        for &piece in Piece::LIST.iter().take(5) {
            let index = Attacks::index(piece);
            let w = self.weights.safety[index];
            write!(output, "{w}, ").unwrap();
        }
        writeln!(output, "\n];",).unwrap();

        write!(
            output,
            "\npub const DEFENSES: [ScoreTuple; (NUM_PIECES - 1) as usize] = [\n  ",
        )
        .unwrap();
        for &piece in Piece::LIST.iter().take(5) {
            let index = Defenses::index(piece);
            let w = self.weights.safety[index];
            write!(output, "{w}, ",).unwrap();
        }
        writeln!(output, "\n];",).unwrap();

        write!(output, "\npub const ENEMY_KING_RANK: Prt = ").unwrap();
        self.write_prt(
            output,
            ";\n",
            self.weights.safety.as_slice(),
            EnemyKingRank::index,
        );
    }

    fn create_output_file(&self) {
        let mut output = BufWriter::new(File::create("eval_constants.rs").unwrap());
        self.write_header(&mut output);
        self.write_material_psts(&mut output);
        self.write_passer_pst(&mut output);
        self.write_passer_blocker_prt(&mut output);
        self.write_isolated_prt(&mut output);
        self.write_phalanx_prt(&mut output);
        self.write_bishop_pair(&mut output);
        self.write_mobility(&mut output);
        self.write_forward_mobility(&mut output);
        self.write_threats(&mut output);
        self.write_tempo(&mut output);

        write!(output, "\n// KING SAFETY FEATURES").unwrap();
        self.write_safety(&mut output);
    }
}
