use crate::{
    bitloop,
    board::attacks,
    board::board_representation::{Bitboard, Board, Color, Piece, Square, NUM_COLORS, NUM_SQUARES},
    eval::eval_constants::{
        BISHOP_FORWARD_MOBILITY, BISHOP_MOBILITY, BISHOP_THREAT_ON_KNIGHT, BISHOP_THREAT_ON_QUEEN,
        BISHOP_THREAT_ON_ROOK, KNIGHT_FORWARD_MOBILITY, KNIGHT_MOBILITY, KNIGHT_THREAT_ON_BISHOP,
        KNIGHT_THREAT_ON_QUEEN, KNIGHT_THREAT_ON_ROOK, PAWN_THREAT_ON_BISHOP,
        PAWN_THREAT_ON_KNIGHT, PAWN_THREAT_ON_QUEEN, PAWN_THREAT_ON_ROOK, QUEEN_FORWARD_MOBILITY,
        QUEEN_MOBILITY, ROOK_FORWARD_MOBILITY, ROOK_MOBILITY, ROOK_THREAT_ON_QUEEN,
    },
    eval::trace::{
        color_adjust, Attacks, Defenses, EnemyKingRank, ForwardMobility, Mobility, Threats, Trace,
    },
    eval::{evaluation::ScoreTuple, trace::Tropism},
    trace_safety_update, trace_threat_update, trace_update,
};

use super::king_safety_net::SafetyNet;

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

const TROPISM_LOOKUP: [[i32; NUM_SQUARES as usize]; NUM_SQUARES as usize] =
    include!(concat!(env!("OUT_DIR"), "/trophism_init.rs"));

const fn king_zone(board: &Board, color: Color) -> Bitboard {
    let king_sq = board.color_king_sq(color);
    KING_ZONES[color.as_index()][king_sq.as_index()]
}

const fn forward_mobility(moves: Bitboard, sq: Square, color: Color) -> usize {
    moves
        .intersection(FORWARD_MASKS[color.as_index()][sq.as_index()])
        .popcount() as usize
}

const fn tropism(king_sq: Square, piece_sq: Square) -> i32 {
    TROPISM_LOOKUP[king_sq.as_index()][piece_sq.as_index()]
}

const fn availible(board: &Board, color: Color) -> Bitboard {
    let opp_color = color.flip();
    let enemy_pawns = board.piece_bb(Piece::PAWN, opp_color);
    let enemy_pawn_attacks = attacks::pawn_setwise(enemy_pawns, opp_color);
    let enemy_or_empty = board.all[opp_color.as_index()].union(board.empty());

    enemy_or_empty.without(enemy_pawn_attacks)
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
    friendly_king_sq: Square,
    enemy_king_sq: Square,
    enemy_king_zone: Bitboard,
    friendly_king_zone: Bitboard,
    enemy_knights: Bitboard,
    enemy_bishops: Bitboard,
    enemy_rooks: Bitboard,
    enemy_queens: Bitboard,
    hv_occupied: Bitboard,
    d12_occupied: Bitboard,

    tropism: i32,
}

impl LoopEvaluator {
    fn new(board: &Board, color: Color) -> Self {
        let opp_color = color.flip();

        let availible = availible(board, color);
        let friendly_king_sq = board.color_king_sq(color);
        let enemy_king_sq = board.color_king_sq(opp_color);
        let friendly_king_zone = king_zone(board, color);
        let enemy_king_zone = king_zone(board, opp_color);

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
            friendly_king_sq,
            enemy_king_sq,
            enemy_king_zone,
            friendly_king_zone,
            enemy_knights,
            enemy_bishops,
            enemy_rooks,
            enemy_queens,
            hv_occupied,
            d12_occupied,
            tropism: 0,
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
    fn single_score<const PIECE: u8, const TRACE: bool>(&mut self, sq: Square, safety_net: &mut [SafetyNet; 2], t: &mut Trace) -> ScoreTuple {
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
        safety_net[color.as_index()].update_attacks(piece, kz_attacks);
        safety_net[opp_color.as_index()].update_defenses(piece, kz_defenses);

        self.tropism += tropism(self.enemy_king_sq, sq);

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
            trace_safety_update!(t, Attacks, (piece), self.color, kz_attacks);
            trace_safety_update!(t, Defenses, (piece), self.color.flip(), kz_defenses);

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
    fn pawn_score<const TRACE: bool>(&self, pawns: Bitboard, color: Color, safety_net: &mut [SafetyNet; 2], t: &mut Trace) -> ScoreTuple {
        let us = color;
        let them = color.flip();
        safety_net[us.as_index()].update_attacking_pawns::<TRACE>(pawns, self.enemy_king_sq, us, t);
        safety_net[them.as_index()].update_defending_pawns::<TRACE>(pawns, self.friendly_king_sq, them, t);

        let pawn_attacks = attacks::pawn_setwise(pawns, color);
        if TRACE {
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
        &mut self,
        mut piece_bb: Bitboard,
        safety_net: &mut [SafetyNet; 2],
        t: &mut Trace,
    ) -> ScoreTuple {
        let mut score = ScoreTuple::new(0, 0);
        bitloop!(|sq| piece_bb, {
            score += self.single_score::<PIECE, TRACE>(sq, safety_net, t);
        });
        score
    }
}

fn one_sided_eval<const TRACE: bool>(
    board: &Board,
    safety_net: &mut [SafetyNet; 2],
    color: Color,
    t: &mut Trace,
) -> ScoreTuple {
    let knights = board.piece_bb(Piece::KNIGHT, color);
    let bishops = board.piece_bb(Piece::BISHOP, color);
    let rooks = board.piece_bb(Piece::ROOK, color);
    let queens = board.piece_bb(Piece::QUEEN, color);
    let pawns = board.piece_bb(Piece::PAWN, color);

    let mut looper = LoopEvaluator::new(board, color);
    let score = looper.piece_loop::<{ ConstPiece::KNIGHT }, TRACE>(knights, safety_net, t)
        + looper.piece_loop::<{ ConstPiece::BISHOP }, TRACE>(bishops, safety_net, t)
        + looper.piece_loop::<{ ConstPiece::ROOK }, TRACE>(rooks, safety_net, t)
        + looper.piece_loop::<{ ConstPiece::QUEEN }, TRACE>(queens, safety_net, t)
        + looper.pawn_score::<TRACE>(pawns, color, safety_net, t);

    let opp_king_sq = looper.enemy_king_sq;
    safety_net[color.as_index()].update_enemy_king_rank(opp_king_sq, color);

    let trop = looper.tropism;
    safety_net[color.as_index()].update_tropism(trop);

    if TRACE {
        let rank = color_adjust(opp_king_sq, color).rank();
        trace_safety_update!(t, EnemyKingRank, (rank), color, 1);

        trace_safety_update!(t, Tropism, (), color, trop);
    }

    score
}

pub fn mobility_threats_safety<const TRACE: bool>(
    board: &Board,
    us: Color,
    them: Color,
    t: &mut Trace,
) -> ScoreTuple {
    let mut safety_net = [SafetyNet::new(), SafetyNet::new()];

    let mobility_and_threats = one_sided_eval::<TRACE>(board, &mut safety_net, us, t)
        - one_sided_eval::<TRACE>(board, &mut safety_net, them, t);

    let safety = safety_net[us.as_index()].calculate() - safety_net[them.as_index()].calculate();

    mobility_and_threats + safety
}

#[cfg(test)]
mod tests {
    use crate::{
        board::attacks,
        board::board_representation::{Board, Color, Square},
        eval::piece_loop_eval::forward_mobility,
    };

    #[test]
    fn forward_mobility_test() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let sq = Square::D3;
        let moves = attacks::queen(sq, board.occupied());
        let f_mobility = forward_mobility(moves, sq, Color::Black);

        assert_eq!(f_mobility, 5);
    }
}
