use engine::{
    attacks, bitloop,
    board_representation::{
        Bitboard, Board, Color, Piece, Square, NUM_PIECES, NUM_RANKS, NUM_SQUARES,
    },
    evaluation::{phase, EvalScore, Phase, EG, MG, NUM_PHASES, PHASES, PHASE_MAX},
    piece_loop_eval::{
        self, enemy_king_zone, enemy_virtual_mobility, forward_mobility, MoveCounts,
    },
};
use std::{
    fs::{read_to_string, File},
    io::BufWriter,
    io::Write,
};

const TUNER_VEC_LEN: usize = MaterialPst::LEN
    + Passer::LEN
    + PasserBlocker::LEN
    + BishopPair::LEN
    + Mobility::LEN
    + Safety::LEN
    + IsolatedPawns::LEN
    + PhalanxPawns::LEN
    + Threats::LEN
    + TempoBonus::LEN
    + ForwardMobility::LEN;
type TunerVec = [[f64; TUNER_VEC_LEN]; NUM_PHASES];

struct MaterialPst;
impl MaterialPst {
    const START: usize = 0;
    const LEN: usize = (NUM_PIECES as usize) * (NUM_SQUARES as usize);

    fn index(piece: Piece, sq: Square) -> usize {
        Self::START + usize::from(NUM_SQUARES) * piece.as_index() + sq.as_index()
    }
}

struct Passer;
impl Passer {
    const START: usize = MaterialPst::START + MaterialPst::LEN;
    const LEN: usize = (NUM_SQUARES as usize);

    fn index(sq: Square) -> usize {
        Self::START + sq.as_index()
    }
}

struct PasserBlocker;
impl PasserBlocker {
    const START: usize = Passer::START + Passer::LEN;
    const LEN: usize = (NUM_RANKS as usize);

    fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

struct BishopPair;
impl BishopPair {
    const START: usize = PasserBlocker::START + PasserBlocker::LEN;
    const LEN: usize = 1;

    fn index() -> usize {
        Self::START
    }
}

struct Mobility;
impl Mobility {
    const START: usize = BishopPair::START + BishopPair::LEN;
    const PIECE_MOVECOUNTS: [usize; 4] = [
        MoveCounts::KNIGHT,
        MoveCounts::BISHOP,
        MoveCounts::ROOK,
        MoveCounts::QUEEN,
    ];
    const PIECE_OFFSETS: [usize; 4] = [
        0,
        MoveCounts::KNIGHT,
        MoveCounts::KNIGHT + MoveCounts::BISHOP,
        MoveCounts::KNIGHT + MoveCounts::BISHOP + MoveCounts::ROOK,
    ];
    const LEN: usize =
        MoveCounts::KNIGHT + MoveCounts::BISHOP + MoveCounts::ROOK + MoveCounts::QUEEN;

    fn index(piece: Piece, attack_count: u32) -> usize {
        Self::START + (attack_count as usize) + Self::PIECE_OFFSETS[piece.as_index()]
    }
}

struct Safety;
impl Safety {
    const START: usize = Mobility::START + Mobility::LEN;
    const LEN: usize = (MoveCounts::QUEEN * (NUM_PIECES - 1) as usize);

    fn index(piece: Piece, enemy_virt_mobility: usize) -> usize {
        Self::START + MoveCounts::QUEEN * piece.as_index() + enemy_virt_mobility
    }
}

struct IsolatedPawns;
impl IsolatedPawns {
    const START: usize = Safety::START + Safety::LEN;
    const LEN: usize = (NUM_RANKS as usize);

    fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

struct PhalanxPawns;
impl PhalanxPawns {
    const START: usize = IsolatedPawns::START + IsolatedPawns::LEN;
    const LEN: usize = (NUM_RANKS as usize);

    fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

struct Threats;
impl Threats {
    const START: usize = PhalanxPawns::START + PhalanxPawns::LEN;
    const LEN: usize = 11;

    const PAWN_THREAT_ON_KNIGHT: usize = Self::START;
    const PAWN_THREAT_ON_BISHOP: usize = Self::START + 1;
    const PAWN_THREAT_ON_ROOK: usize = Self::START + 2;
    const PAWN_THREAT_ON_QUEEN: usize = Self::START + 3;

    const KNIGHT_THREAT_ON_BISHOP: usize = Self::START + 4;
    const KNIGHT_THREAT_ON_ROOK: usize = Self::START + 5;
    const KNIGHT_THREAT_ON_QUEEN: usize = Self::START + 6;

    const BISHOP_THREAT_ON_KNIGHT: usize = Self::START + 7;
    const BISHOP_THREAT_ON_ROOK: usize = Self::START + 8;
    const BISHOP_THREAT_ON_QUEEN: usize = Self::START + 9;

    const ROOK_THREAT_ON_QUEEN: usize = Self::START + 10;
}

struct TempoBonus;
impl TempoBonus {
    const START: usize = Threats::START + Threats::LEN;
    const LEN: usize = (NUM_RANKS as usize);

    fn index() -> usize {
        Self::START
    }
}

struct ForwardMobility;
impl ForwardMobility {
    const START: usize = TempoBonus::START + TempoBonus::LEN;
    const PIECE_MOVECOUNTS: [usize; 4] = [
        MoveCounts::FORWARD_KNIGHT,
        MoveCounts::FORWARD_BISHOP,
        MoveCounts::FORWARD_ROOK,
        MoveCounts::FORWARD_QUEEN,
    ];
    const PIECE_OFFSETS: [usize; 4] = [
        0,
        MoveCounts::FORWARD_KNIGHT,
        MoveCounts::FORWARD_KNIGHT + MoveCounts::FORWARD_BISHOP,
        MoveCounts::FORWARD_KNIGHT + MoveCounts::FORWARD_BISHOP + MoveCounts::FORWARD_ROOK,
    ];
    const LEN: usize = MoveCounts::FORWARD_KNIGHT
        + MoveCounts::FORWARD_BISHOP
        + MoveCounts::FORWARD_ROOK
        + MoveCounts::FORWARD_QUEEN;

    fn index(piece: Piece, f_mobility: usize) -> usize {
        Self::START + f_mobility + Self::PIECE_OFFSETS[piece.as_index()]
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
    pub fn pst_update<F>(&mut self, w_pieces: Bitboard, b_pieces: Bitboard, index_fn: F)
    where
        F: Fn(Square) -> usize,
    {
        for i in 0..NUM_SQUARES {
            let sq = Square::new(i);
            let w_sq = sq.flip();
            let b_sq = sq;

            let value = (w_sq.as_bitboard().intersection(w_pieces).popcount() as i8)
                - (b_sq.as_bitboard().intersection(b_pieces).popcount() as i8);
            if value != 0 {
                self.feature_vec.push(Feature::new(value, index_fn(sq)));
            }
        }
    }

    pub fn prt_update<F>(&mut self, w_pieces: Bitboard, b_pieces: Bitboard, index_fn: F)
    where
        F: Fn(u8) -> usize,
    {
        let rank = Bitboard::RANK_1;
        for i in 0..NUM_RANKS {
            let w_rank = rank.shift_north(7 - i);
            let b_rank = rank.shift_north(i);

            let value = (w_rank.intersection(w_pieces).popcount() as i8)
                - (b_rank.intersection(b_pieces).popcount() as i8);
            if value != 0 {
                self.feature_vec.push(Feature::new(value, index_fn(i)));
            }
        }
    }

    fn add_pst_features(&mut self, board: &Board) {
        for piece in Piece::LIST {
            let w_piece_bb = board.piece_bb(piece, Color::White);
            let b_piece_bb = board.piece_bb(piece, Color::Black);
            self.pst_update(w_piece_bb, b_piece_bb, |sq| MaterialPst::index(piece, sq));
        }
    }

    fn add_passer_features(&mut self, board: &Board) {
        let w_passers = board.passed_pawns(Color::White);
        let b_passers = board.passed_pawns(Color::Black);
        self.pst_update(w_passers, b_passers, Passer::index);

        let blocking_white = w_passers
            .north_one()
            .intersection(board.all[Color::Black.as_index()]);
        let blocking_black = b_passers
            .south_one()
            .intersection(board.all[Color::White.as_index()]);
        self.prt_update(blocking_white, blocking_black, PasserBlocker::index);
    }

    fn add_isolated_features(&mut self, board: &Board) {
        let w_isolated = board.isolated_pawns(Color::White);
        let b_isolated = board.isolated_pawns(Color::Black);

        self.prt_update(w_isolated, b_isolated, IsolatedPawns::index);
    }

    fn add_phalanx_features(&mut self, board: &Board) {
        let w_phalanx = board.phalanx_pawns(Color::White);
        let b_phalanx = board.phalanx_pawns(Color::Black);

        self.prt_update(w_phalanx, b_phalanx, PhalanxPawns::index);
    }

    fn add_threat_val(
        board: &Board,
        piece: Piece,
        attacks: Bitboard,
        threats: &mut [i8; Threats::LEN],
        color: Color,
    ) {
        let sign = if color == Color::White { 1 } else { -1 };
        let offset = Threats::START;

        let knights = board.piece_bb(Piece::KNIGHT, color.flip());
        let bishops = board.piece_bb(Piece::BISHOP, color.flip());
        let rooks = board.piece_bb(Piece::ROOK, color.flip());
        let queens = board.piece_bb(Piece::QUEEN, color.flip());
        match piece {
            Piece::KNIGHT => {
                threats[Threats::KNIGHT_THREAT_ON_BISHOP - offset] +=
                    sign * (attacks & bishops).popcount() as i8;
                threats[Threats::KNIGHT_THREAT_ON_ROOK - offset] +=
                    sign * (attacks & rooks).popcount() as i8;
                threats[Threats::KNIGHT_THREAT_ON_QUEEN - offset] +=
                    sign * (attacks & queens).popcount() as i8;
            }
            Piece::BISHOP => {
                threats[Threats::BISHOP_THREAT_ON_KNIGHT - offset] +=
                    sign * (attacks & knights).popcount() as i8;
                threats[Threats::BISHOP_THREAT_ON_ROOK - offset] +=
                    sign * (attacks & rooks).popcount() as i8;
                threats[Threats::BISHOP_THREAT_ON_QUEEN - offset] +=
                    sign * (attacks & queens).popcount() as i8;
            }
            Piece::ROOK => {
                threats[Threats::ROOK_THREAT_ON_QUEEN - offset] +=
                    sign * (attacks & queens).popcount() as i8;
            }
            Piece::PAWN => {
                threats[Threats::PAWN_THREAT_ON_KNIGHT - offset] +=
                    sign * (attacks & knights).popcount() as i8;
                threats[Threats::PAWN_THREAT_ON_BISHOP - offset] +=
                    sign * (attacks & bishops).popcount() as i8;
                threats[Threats::PAWN_THREAT_ON_ROOK - offset] +=
                    sign * (attacks & rooks).popcount() as i8;
                threats[Threats::PAWN_THREAT_ON_QUEEN - offset] +=
                    sign * (attacks & queens).popcount() as i8;
            }
            _ => (),
        }
    }

    fn add_piece_loop_features(&mut self, board: &Board) {
        let mut mobility = [0; Mobility::LEN];
        let mut f_mobility = [0; ForwardMobility::LEN];
        let mut safety = [0; Safety::LEN];
        let mut threats = [0; Threats::LEN];

        for color in Color::LIST {
            let availible = piece_loop_eval::availible(board, color);
            let enemy_king_virt_mobility = enemy_virtual_mobility(board, color);

            let mult = match color {
                Color::White => 1,
                Color::Black => -1,
            };

            for &piece in Piece::LIST.iter().take(4) {
                let mut pieces = board.piece_bb(piece, color);

                bitloop!(|sq| pieces, {
                    let attacks = attacks::generic(piece, sq, board.occupied(), color) & availible;
                    let count = attacks.popcount();
                    if count > 0 {
                        mobility[Mobility::index(piece, count) - Mobility::START] += mult;
                    }

                    let forward_count = forward_mobility(attacks, sq, color);
                    if forward_count > 0 {
                        f_mobility[ForwardMobility::index(piece, forward_count)
                            - ForwardMobility::START] += mult;
                    }

                    let kz_attacks = (attacks & enemy_king_zone(board, color)).popcount() as i8;
                    safety[Safety::index(piece, enemy_king_virt_mobility) - Safety::START] +=
                        kz_attacks * mult;

                    Self::add_threat_val(board, piece, attacks, &mut threats, color);
                });
            }

            let pawns = board.piece_bb(Piece::PAWN, color);
            let pawn_attacks = attacks::pawn_setwise(pawns, color);
            let kz_attacks = (pawn_attacks & enemy_king_zone(board, color)).popcount() as i8;
            safety[Safety::index(Piece::PAWN, enemy_king_virt_mobility) - Safety::START] +=
                kz_attacks * mult;

            Self::add_threat_val(board, Piece::PAWN, pawn_attacks, &mut threats, color);
        }

        for i in 0..Mobility::LEN {
            let val = mobility[i];
            if val != 0 {
                let vec_index = i + Mobility::START;
                self.feature_vec.push(Feature::new(val, vec_index));
            }
        }

        // todo: make all these a macro :p
        for i in 0..ForwardMobility::LEN {
            let val = f_mobility[i];
            if val != 0 {
                let vec_index = i + ForwardMobility::START;
                self.feature_vec.push(Feature::new(val, vec_index));
            }
        }

        for i in 0..Safety::LEN {
            let val = safety[i];
            if val != 0 {
                let vec_index = i + Safety::START;
                self.feature_vec.push(Feature::new(val, vec_index));
            }
        }

        for i in 0..Threats::LEN {
            let val = threats[i];
            if val != 0 {
                let vec_index = i + Threats::START;
                self.feature_vec.push(Feature::new(val, vec_index));
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
        entry.add_passer_features(board);
        entry.add_piece_loop_features(board);
        entry.add_isolated_features(board);
        entry.add_phalanx_features(board);

        let bishop_pair_val = i8::from(board.piece_bb(Piece::BISHOP, Color::White).popcount() >= 2)
            - i8::from(board.piece_bb(Piece::BISHOP, Color::Black).popcount() >= 2);
        entry
            .feature_vec
            .push(Feature::new(bishop_pair_val, BishopPair::index()));

        let tempo = match board.color_to_move {
            Color::White => 1,
            Color::Black => -1,
        };
        entry
            .feature_vec
            .push(Feature::new(tempo, TempoBonus::index()));

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

    fn new_weights(from_zero: bool) -> TunerVec {
        if from_zero {
            return [[0.0; TUNER_VEC_LEN]; NUM_PHASES];
        }

        let scores: [EvalScore; NUM_PIECES as usize] = [300, 320, 500, 900, 100, 0];
        let mut result = [[0.0; TUNER_VEC_LEN]; NUM_PHASES];

        for piece in Piece::LIST {
            for i in 0..NUM_SQUARES {
                let index = MaterialPst::index(piece, Square::new(i));
                result[MG][index] = scores[piece.as_index()] as f64;
                result[EG][index] = scores[piece.as_index()] as f64;
            }
        }
        result
    }

    pub fn new(from_zero: bool) -> Self {
        Self {
            entries: vec![],
            gradient: [[0.0; TUNER_VEC_LEN]; NUM_PHASES],
            weights: Self::new_weights(from_zero),
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
            "use crate::{{evaluation::ScoreTuple, board_representation::NUM_PIECES, pst::{{Pst, Prt}}}};\n"
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
                let index = Mobility::index(piece, i.try_into().unwrap());
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
        self.write_bishop_pair(&mut output);
        self.write_mobility(&mut output);
        self.write_forward_mobility(&mut output);
        self.write_safety(&mut output);
        self.write_threats(&mut output);
        self.write_tempo(&mut output);
    }
}
