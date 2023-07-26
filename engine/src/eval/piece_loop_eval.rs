use crate::{
    bitloop,
    board::attacks,
    board::board_representation::{Bitboard, Board, Color, Piece, Square, NUM_COLORS, NUM_SQUARES},
    eval::eval_constants::{
        ATTACKS, BISHOP_FORWARD_MOBILITY, BISHOP_MOBILITY, BISHOP_THREAT_ON_KNIGHT,
        BISHOP_THREAT_ON_QUEEN, BISHOP_THREAT_ON_ROOK, DEFENSES, ENEMY_KING_PST,
        KNIGHT_FORWARD_MOBILITY, KNIGHT_MOBILITY, KNIGHT_THREAT_ON_BISHOP, KNIGHT_THREAT_ON_QUEEN,
        KNIGHT_THREAT_ON_ROOK, PAWN_THREAT_ON_BISHOP, PAWN_THREAT_ON_KNIGHT, PAWN_THREAT_ON_QUEEN,
        PAWN_THREAT_ON_ROOK, QUEEN_FORWARD_MOBILITY, QUEEN_MOBILITY, ROOK_FORWARD_MOBILITY,
        ROOK_MOBILITY, ROOK_THREAT_ON_QUEEN,
    },
    eval::evaluation::ScoreTuple,
    eval::trace::{
        color_adjust, Attacks, Defenses, EnemyKingPst, ForwardMobility, Mobility, Threats, Trace,
    },
    trace_safety_update, trace_threat_update, trace_update,
};

const fn king_zones_init() -> [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] {
    let mut king_zones = [[Bitboard::EMPTY; NUM_SQUARES as usize]; NUM_COLORS as usize];
    let mut i = 0;
    while i < NUM_SQUARES {
        let sq = Square::new(i);
        let adjusted_sq = Square::new(i + (i % 8 == 0) as u8 - (i % 8 == 7) as u8);
        let inner_ring = attacks::king(adjusted_sq)
            .union(adjusted_sq.as_bitboard())
            .xor(sq.as_bitboard());

        let bitset = adjusted_sq.as_bitboard();

        let w_shield = bitset
            .northwest_one()
            .union(bitset.north_one())
            .union(bitset.northeast_one());
        let b_shield = bitset
            .southwest_one()
            .union(bitset.south_one())
            .union(bitset.southeast_one());

        let white_zone = inner_ring
            .union(w_shield.shift_north(1))
            .union(w_shield.shift_north(2));

        let black_zone = inner_ring
            .union(b_shield.shift_south(1))
            .union(b_shield.shift_south(2));

        king_zones[Color::White.as_index()][sq.as_index()] = white_zone;
        king_zones[Color::Black.as_index()][sq.as_index()] = black_zone;
        i += 1;
    }

    king_zones
}

const fn forward_masks_init() -> [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] {
    let mut result = [[Bitboard::EMPTY; NUM_SQUARES as usize]; NUM_COLORS as usize];

    let mut i = 0;
    while i < NUM_SQUARES {
        let sq = Square::new(i);
        let rank_bb = Bitboard::RANK_1.shift_north(sq.rank());

        let w_backwards = rank_bb.fill(Color::Black);
        let b_backwards = rank_bb.fill(Color::White);

        let moves = attacks::knight(sq).union(attacks::queen(sq, Bitboard::EMPTY));
        result[Color::White.as_index()][sq.as_index()] = moves.without(w_backwards);
        result[Color::Black.as_index()][sq.as_index()] = moves.without(b_backwards);

        i += 1;
    }

    result
}

const KING_ZONES: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] = king_zones_init();

const FORWARD_MASKS: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] = forward_masks_init();

pub const fn king_zone(board: &Board, color: Color) -> Bitboard {
    let king_sq = board.color_king_sq(color);
    KING_ZONES[color.as_index()][king_sq.as_index()]
}

pub const fn forward_mobility(moves: Bitboard, sq: Square, color: Color) -> usize {
    moves
        .intersection(FORWARD_MASKS[color.as_index()][sq.as_index()])
        .popcount() as usize
}

pub const fn availible(board: &Board, color: Color) -> Bitboard {
    let opp_color = color.flip();
    let enemy_pawns = board.piece_bb(Piece::PAWN, opp_color);
    let enemy_pawn_attacks = attacks::pawn_setwise(enemy_pawns, opp_color);
    let enemy_or_empty = board.all[opp_color.as_index()].union(board.empty());

    enemy_or_empty.without(enemy_pawn_attacks)
}

pub fn virtual_mobility(board: &Board, color: Color) -> usize {
    let opp_color = color.flip();
    let king_sq = board.color_king_sq(color);
    let empty = board.empty();
    let mobile_attacking_pieces =
        board.all[opp_color.as_index()] ^ board.piece_bb(Piece::PAWN, opp_color);
    let virtual_occupied = board.occupied() ^ mobile_attacking_pieces;
    let attackers_or_empty = board.all[opp_color.as_index()].union(empty);

    (attacks::queen(king_sq, virtual_occupied) & attackers_or_empty).popcount() as usize
}

pub struct MoveCounts;
impl MoveCounts {
    pub const KNIGHT: usize = 9;
    pub const BISHOP: usize = 14;
    pub const ROOK: usize = 15;
    pub const QUEEN: usize = 28;

    pub const FORWARD_KNIGHT: usize = 5;
    pub const FORWARD_BISHOP: usize = 8;
    pub const FORWARD_ROOK: usize = 8;
    pub const FORWARD_QUEEN: usize = 15;
}

struct ConstPiece;
impl ConstPiece {
    const KNIGHT: u8 = 0;
    const BISHOP: u8 = 1;
    const ROOK: u8 = 2;
    const QUEEN: u8 = 3;

    const fn piece<const PIECE: u8>() -> Piece {
        match PIECE {
            Self::KNIGHT => Piece::KNIGHT,
            Self::BISHOP => Piece::BISHOP,
            Self::ROOK => Piece::ROOK,
            Self::QUEEN => Piece::QUEEN,
            _ => panic!("Unexpected Piece!"),
        }
    }
}

struct LoopEvaluator {
    color: Color,
    availible: Bitboard,
    enemy_king_zone: Bitboard,
    friendly_king_zone: Bitboard,
    own_virt_mob: usize,
    enemy_virt_mob: usize,
    enemy_knights: Bitboard,
    enemy_bishops: Bitboard,
    enemy_rooks: Bitboard,
    enemy_queens: Bitboard,
    hv_occupied: Bitboard,
    d12_occupied: Bitboard,
}

impl LoopEvaluator {
    fn new(board: &Board, own_virt_mob: usize, enemy_virt_mob: usize, color: Color) -> Self {
        let availible = availible(board, color);
        let friendly_king_zone = king_zone(board, color);
        let enemy_king_zone = king_zone(board, color.flip());

        let opp_color = color.flip();
        let enemy_knights = board.piece_bb(Piece::KNIGHT, opp_color);
        let enemy_bishops = board.piece_bb(Piece::BISHOP, opp_color);
        let enemy_rooks = board.piece_bb(Piece::ROOK, opp_color);
        let enemy_queens = board.piece_bb(Piece::QUEEN, opp_color);

        let occ = board.occupied();
        let hv_sliders = board.piece_bb(Piece::ROOK, color) | board.piece_bb(Piece::QUEEN, color);
        let d12_sliders =
            board.piece_bb(Piece::BISHOP, color) | board.piece_bb(Piece::QUEEN, color);
        let hv_occupied = occ ^ hv_sliders;
        let d12_occupied = occ ^ d12_sliders;
        Self {
            color,
            availible,
            enemy_king_zone,
            friendly_king_zone,
            own_virt_mob,
            enemy_virt_mob,
            enemy_knights,
            enemy_bishops,
            enemy_rooks,
            enemy_queens,
            hv_occupied,
            d12_occupied,
        }
    }
    const fn moves<const PIECE: u8>(&self, sq: Square) -> Bitboard {
        match PIECE {
            ConstPiece::KNIGHT => attacks::knight(sq),
            ConstPiece::BISHOP => attacks::bishop(sq, self.d12_occupied),
            ConstPiece::ROOK => attacks::rook(sq, self.hv_occupied),
            ConstPiece::QUEEN => {
                attacks::bishop(sq, self.d12_occupied).union(attacks::rook(sq, self.hv_occupied))
            }
            _ => panic!("Unexpected Piece!"),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[rustfmt::skip]
    fn single_score<const PIECE: u8, const TRACE: bool>(&self, sq: Square, attack_power: &mut [ScoreTuple; 2], t: &mut Trace) -> ScoreTuple {
        let mut score = ScoreTuple::new(0, 0);
        let color = self.color;
        let opp_color = self.color.flip();
        let piece = ConstPiece::piece::<PIECE>();
        let attacks = self.moves::<PIECE>(sq);
        let moves = attacks & self.availible;

        let mobility = moves.popcount() as usize;
        let forward_mobility = forward_mobility(moves, sq, self.color);

        let kz_attacks = self.enemy_king_zone.intersection(moves).popcount() as i32;
        let kz_defenses = self.friendly_king_zone.intersection(moves).popcount() as i32;
        attack_power[color.as_index()] += ATTACKS[piece.as_index()][self.enemy_virt_mob].mult(kz_attacks);
        attack_power[opp_color.as_index()] += DEFENSES[piece.as_index()][self.own_virt_mob].mult(kz_defenses);

        match PIECE {
            ConstPiece::KNIGHT => {
                score += KNIGHT_MOBILITY[mobility];
                score += KNIGHT_FORWARD_MOBILITY[forward_mobility];

                score += KNIGHT_THREAT_ON_BISHOP
                    .mult((attacks & self.enemy_bishops).popcount() as i32)
                    + KNIGHT_THREAT_ON_ROOK.mult((attacks & self.enemy_rooks).popcount() as i32)
                    + KNIGHT_THREAT_ON_QUEEN.mult((attacks & self.enemy_queens).popcount() as i32);

                if TRACE {
                    trace_threat_update!(t, KNIGHT_THREAT_ON_BISHOP, color, attacks, self.enemy_bishops);
                    trace_threat_update!(t, KNIGHT_THREAT_ON_ROOK, color, attacks, self.enemy_rooks);
                    trace_threat_update!(t, KNIGHT_THREAT_ON_QUEEN, color, attacks, self.enemy_queens);
                }
            }
            ConstPiece::BISHOP => {
                score += BISHOP_MOBILITY[mobility];
                score += BISHOP_FORWARD_MOBILITY[forward_mobility];

                score += BISHOP_THREAT_ON_KNIGHT
                    .mult((attacks & self.enemy_knights).popcount() as i32)
                    + BISHOP_THREAT_ON_ROOK.mult((attacks & self.enemy_rooks).popcount() as i32)
                    + BISHOP_THREAT_ON_QUEEN.mult((attacks & self.enemy_queens).popcount() as i32);

                    if TRACE {
                        trace_threat_update!(t, BISHOP_THREAT_ON_KNIGHT, color, attacks, self.enemy_knights);
                        trace_threat_update!(t, BISHOP_THREAT_ON_ROOK, color, attacks, self.enemy_rooks);
                        trace_threat_update!(t, BISHOP_THREAT_ON_QUEEN, color, attacks, self.enemy_queens);
                    }
            }
            ConstPiece::ROOK => {
                score += ROOK_MOBILITY[mobility];
                score += ROOK_FORWARD_MOBILITY[forward_mobility];

                score += ROOK_THREAT_ON_QUEEN.mult((attacks & self.enemy_queens).popcount() as i32);

                if TRACE {
                    trace_threat_update!(t, ROOK_THREAT_ON_QUEEN, color, attacks, self.enemy_queens);
                }
            }
            ConstPiece::QUEEN => {
                score += QUEEN_MOBILITY[mobility];
                score += QUEEN_FORWARD_MOBILITY[forward_mobility];
            }
            _ => (),
        }

        if TRACE {
            let (own_vm, enemy_vm) = (self.own_virt_mob, self.enemy_virt_mob);
            trace_safety_update!(t, Attacks, (piece, enemy_vm), self.color, kz_attacks);
            trace_safety_update!(t, Defenses, (piece, own_vm), self.color.flip(), kz_defenses);

            // we fix 0 mobility at 0 for eval constants readability
            if mobility > 0 {
                trace_update!(t, Mobility, (piece, mobility), self.color, 1);
            }
            if forward_mobility > 0 {
                trace_update!(t, ForwardMobility, (piece, forward_mobility), self.color, 1);
            }
        }

        score
    }

    #[allow(clippy::cast_possible_wrap)]
    #[rustfmt::skip]
    fn pawn_score<const TRACE: bool>(&self, pawns: Bitboard, color: Color, attack_power: &mut [ScoreTuple; 2], t: &mut Trace) -> ScoreTuple {
        let piece = Piece::PAWN;
        let pawn_attacks = attacks::pawn_setwise(pawns, color);
        let kz_attacks = self.enemy_king_zone.intersection(pawn_attacks).popcount() as i32;
        let kz_defenses = self.friendly_king_zone.intersection(pawn_attacks).popcount() as i32;
        attack_power[color.as_index()] += ATTACKS[piece.as_index()][self.enemy_virt_mob].mult(kz_attacks);
        attack_power[color.flip().as_index()] += DEFENSES[piece.as_index()][self.own_virt_mob].mult(kz_defenses);

        if TRACE {
            let (own_vm, enemy_vm) = (self.own_virt_mob, self.enemy_virt_mob);
            trace_safety_update!(t, Attacks, (piece, enemy_vm), self.color, kz_attacks);
            trace_safety_update!(t, Defenses, (piece, own_vm), self.color.flip(), kz_defenses);

            trace_threat_update!(t, PAWN_THREAT_ON_KNIGHT, self.color, pawn_attacks, self.enemy_knights);
            trace_threat_update!(t, PAWN_THREAT_ON_BISHOP, self.color, pawn_attacks, self.enemy_bishops);
            trace_threat_update!(t, PAWN_THREAT_ON_ROOK, self.color, pawn_attacks, self.enemy_rooks);
            trace_threat_update!(t, PAWN_THREAT_ON_QUEEN, self.color, pawn_attacks, self.enemy_queens);
        }

        PAWN_THREAT_ON_KNIGHT.mult((pawn_attacks & self.enemy_knights).popcount() as i32)
            + PAWN_THREAT_ON_BISHOP.mult((pawn_attacks & self.enemy_bishops).popcount() as i32)
            + PAWN_THREAT_ON_ROOK.mult((pawn_attacks & self.enemy_rooks).popcount() as i32)
            + PAWN_THREAT_ON_QUEEN.mult((pawn_attacks & self.enemy_queens).popcount() as i32)
    }

    fn piece_loop<const PIECE: u8, const TRACE: bool>(
        &self,
        mut piece_bb: Bitboard,
        attack_power: &mut [ScoreTuple; 2],
        t: &mut Trace,
    ) -> ScoreTuple {
        let mut score = ScoreTuple::new(0, 0);
        bitloop!(|sq| piece_bb, {
            score += self.single_score::<PIECE, TRACE>(sq, attack_power, t);
        });
        score
    }
}

pub fn one_sided_eval<const TRACE: bool>(
    board: &Board,
    attack_power: &mut [ScoreTuple; 2],
    own_virt_mob: usize,
    enemy_virt_mob: usize,
    color: Color,
    t: &mut Trace,
) -> ScoreTuple {
    let opp_king_sq = board.color_king_sq(color.flip());
    attack_power[color.as_index()] += ENEMY_KING_PST.access(color, opp_king_sq);

    if TRACE {
        let sq = color_adjust(opp_king_sq, color);
        trace_safety_update!(t, EnemyKingPst, (sq), color, 1);
    }

    let knights = board.piece_bb(Piece::KNIGHT, color);
    let bishops = board.piece_bb(Piece::BISHOP, color);
    let rooks = board.piece_bb(Piece::ROOK, color);
    let queens = board.piece_bb(Piece::QUEEN, color);
    let pawns = board.piece_bb(Piece::PAWN, color);

    let looper = LoopEvaluator::new(board, own_virt_mob, enemy_virt_mob, color);
    looper.piece_loop::<{ ConstPiece::KNIGHT }, TRACE>(knights, attack_power, t)
        + looper.piece_loop::<{ ConstPiece::BISHOP }, TRACE>(bishops, attack_power, t)
        + looper.piece_loop::<{ ConstPiece::ROOK }, TRACE>(rooks, attack_power, t)
        + looper.piece_loop::<{ ConstPiece::QUEEN }, TRACE>(queens, attack_power, t)
        + looper.pawn_score::<TRACE>(pawns, color, attack_power, t)
}

pub fn mobility_threats_safety<const TRACE: bool>(
    board: &Board,
    us: Color,
    them: Color,
    t: &mut Trace,
) -> ScoreTuple {
    let mut attack_power = [ScoreTuple::new(0, 0), ScoreTuple::new(0, 0)];

    let us_virt_mob = virtual_mobility(board, us);
    let them_virt_mob = virtual_mobility(board, them);

    let mobility_and_threats =
        one_sided_eval::<TRACE>(board, &mut attack_power, us_virt_mob, them_virt_mob, us, t)
            - one_sided_eval::<TRACE>(
                board,
                &mut attack_power,
                them_virt_mob,
                us_virt_mob,
                them,
                t,
            );

    let safety = attack_power[us.as_index()].king_safety_formula()
        - attack_power[them.as_index()].king_safety_formula();

    mobility_and_threats + safety
}

#[cfg(test)]
mod tests {
    use crate::{
        board::attacks,
        board::board_representation::{Board, Color, Piece, Square},
        eval::{
            evaluation::trace_of_position,
            piece_loop_eval::{forward_mobility, virtual_mobility},
            trace::{Attacks, Defenses, EnemyKingPst, SAFETY_TRACE_LEN},
        },
    };

    #[test]
    fn virtual_mobility_test() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let w_virt_mobility = virtual_mobility(&board, Color::White);
        let b_virt_mobility = virtual_mobility(&board, Color::Black);

        assert_eq!(w_virt_mobility, 2);
        assert_eq!(b_virt_mobility, 5);
    }

    #[test]
    fn forward_mobility_test() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let sq = Square::D3;
        let moves = attacks::queen(sq, board.occupied());
        let f_mobility = forward_mobility(moves, sq, Color::Black);

        assert_eq!(f_mobility, 5);
    }

    #[test]
    fn safety_trace_test() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let actual = trace_of_position(&board);
        let (mut w, mut b) = ([0; SAFETY_TRACE_LEN], [0; SAFETY_TRACE_LEN]);

        let w_virt_mob = 2;
        let b_virt_mob = 5;

        w[Attacks::index(Piece::BISHOP, b_virt_mob)] += 3;
        w[Attacks::index(Piece::PAWN, b_virt_mob)] += 1;

        b[Attacks::index(Piece::BISHOP, w_virt_mob)] += 1;
        b[Attacks::index(Piece::PAWN, w_virt_mob)] += 2;
        b[Attacks::index(Piece::QUEEN, w_virt_mob)] += 2;

        b[Defenses::index(Piece::BISHOP, w_virt_mob)] += 3;
        b[Defenses::index(Piece::PAWN, w_virt_mob)] += 3;
        b[Defenses::index(Piece::KNIGHT, w_virt_mob)] += 2;
        b[Defenses::index(Piece::ROOK, w_virt_mob)] += 4;

        w[Defenses::index(Piece::ROOK, b_virt_mob)] += 1;
        w[Defenses::index(Piece::PAWN, b_virt_mob)] += 3;

        let w_k_sq = board.color_king_sq(Color::White);
        let b_k_sq = board.color_king_sq(Color::Black);
        b[EnemyKingPst::index(w_k_sq)] += 1;
        w[EnemyKingPst::index(b_k_sq.flip())] += 1;

        for color in Color::LIST {
            let actual = actual.safety[color.as_index()];
            let expected = [w, b][color.as_index()];
            for (i, (ac, ex)) in actual.iter().zip(expected.iter()).enumerate() {
                assert_eq!(
                    ac,
                    ex,
                    "Expected {ex}, found {ac} at index {i} and color {}",
                    color.as_index()
                );
            }
        }
    }
}
