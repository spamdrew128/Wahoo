use crate::{
    bitloop,
    board::attacks,
    board::board_representation::{Bitboard, Board, Color, Piece, Square, NUM_COLORS, NUM_SQUARES},
    eval::eval_constants::{
        BISHOP_FORWARD_MOBILITY, BISHOP_MOBILITY, BISHOP_THREAT_ON_KNIGHT, BISHOP_THREAT_ON_QUEEN,
        BISHOP_THREAT_ON_ROOK, KNIGHT_FORWARD_MOBILITY, KNIGHT_MOBILITY,
        KNIGHT_THREAT_ON_BISHOP, KNIGHT_THREAT_ON_QUEEN, KNIGHT_THREAT_ON_ROOK,
        PAWN_THREAT_ON_BISHOP, PAWN_THREAT_ON_KNIGHT, PAWN_THREAT_ON_QUEEN, PAWN_THREAT_ON_ROOK,
        QUEEN_FORWARD_MOBILITY, QUEEN_MOBILITY, ROOK_FORWARD_MOBILITY, ROOK_MOBILITY,
        ROOK_THREAT_ON_QUEEN,
    },
    eval::evaluation::ScoreTuple,
    eval::trace::{ForwardMobility, Mobility, Threats, Trace},
    trace_threat_update, trace_update,
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

const KING_ZONES: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] =
    king_zones_init();

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

pub fn enemy_virtual_mobility(board: &Board, color: Color) -> usize {
    let king_sq = board.color_king_sq(color.flip());
    let empty = board.empty();
    let mobile_attacking_pieces = board.all[color.as_index()] ^ board.piece_bb(Piece::PAWN, color);
    let virtual_occupied = board.occupied() ^ mobile_attacking_pieces;
    let attackers_or_empty = board.all[color.as_index()].union(empty);

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

    const fn moves<const PIECE: u8>(board: &Board, sq: Square) -> Bitboard {
        match PIECE {
            Self::KNIGHT => attacks::knight(sq),
            Self::BISHOP => attacks::bishop(sq, board.occupied()),
            Self::ROOK => attacks::rook(sq, board.occupied()),
            Self::QUEEN => attacks::queen(sq, board.occupied()),
            _ => panic!("Unexpected Piece!"),
        }
    }

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
    enemy_virt_mobility: usize,
    enemy_knights: Bitboard,
    enemy_bishops: Bitboard,
    enemy_rooks: Bitboard,
    enemy_queens: Bitboard,
}

impl LoopEvaluator {
    fn new(board: &Board, color: Color) -> Self {
        let availible = availible(board, color);
        let friendly_king_zone = king_zone(board, color);
        let enemy_king_zone = king_zone(board, color.flip());
        let enemy_virt_mobility = enemy_virtual_mobility(board, color);

        let opp_color = color.flip();
        let enemy_knights = board.piece_bb(Piece::KNIGHT, opp_color);
        let enemy_bishops = board.piece_bb(Piece::BISHOP, opp_color);
        let enemy_rooks = board.piece_bb(Piece::ROOK, opp_color);
        let enemy_queens = board.piece_bb(Piece::QUEEN, opp_color);

        Self {
            color,
            availible,
            enemy_king_zone,
            friendly_king_zone,
            enemy_virt_mobility,
            enemy_knights,
            enemy_bishops,
            enemy_rooks,
            enemy_queens,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[rustfmt::skip]
    fn single_score<const PIECE: u8, const TRACE: bool>(&self, board: &Board, sq: Square, t: &mut Trace) -> ScoreTuple {
        let mut score = ScoreTuple::new(0, 0);
        let piece = ConstPiece::piece::<PIECE>();
        let attacks = ConstPiece::moves::<PIECE>(board, sq);
        let moves = attacks & self.availible;

        let mobility = moves.popcount() as usize;
        let forward_mobility = forward_mobility(moves, sq, self.color);

        let kz_attacks = self.enemy_king_zone.intersection(moves).popcount() as i32;
        let attack_weight = KING_ZONE_ATTACKS[piece.as_index()][self.enemy_virt_mobility];
        score += attack_weight.mult(kz_attacks);

        match PIECE {
            ConstPiece::KNIGHT => {
                score += KNIGHT_MOBILITY[mobility];
                score += KNIGHT_FORWARD_MOBILITY[forward_mobility];

                score += KNIGHT_THREAT_ON_BISHOP
                    .mult((attacks & self.enemy_bishops).popcount() as i32)
                    + KNIGHT_THREAT_ON_ROOK.mult((attacks & self.enemy_rooks).popcount() as i32)
                    + KNIGHT_THREAT_ON_QUEEN.mult((attacks & self.enemy_queens).popcount() as i32);

                if TRACE {
                    trace_threat_update!(t, KNIGHT_THREAT_ON_BISHOP, self.color, attacks, self.enemy_bishops);
                    trace_threat_update!(t, KNIGHT_THREAT_ON_ROOK, self.color, attacks, self.enemy_rooks);
                    trace_threat_update!(t, KNIGHT_THREAT_ON_QUEEN, self.color, attacks, self.enemy_queens);
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
                        trace_threat_update!(t, BISHOP_THREAT_ON_KNIGHT, self.color, attacks, self.enemy_knights);
                        trace_threat_update!(t, BISHOP_THREAT_ON_ROOK, self.color, attacks, self.enemy_rooks);
                        trace_threat_update!(t, BISHOP_THREAT_ON_QUEEN, self.color, attacks, self.enemy_queens);
                    }
            }
            ConstPiece::ROOK => {
                score += ROOK_MOBILITY[mobility];
                score += ROOK_FORWARD_MOBILITY[forward_mobility];

                score += ROOK_THREAT_ON_QUEEN.mult((attacks & self.enemy_queens).popcount() as i32);

                if TRACE {
                    trace_threat_update!(t, ROOK_THREAT_ON_QUEEN, self.color, attacks, self.enemy_queens);
                }
            }
            ConstPiece::QUEEN => {
                score += QUEEN_MOBILITY[mobility];
                score += QUEEN_FORWARD_MOBILITY[forward_mobility];
            }
            _ => (),
        }

        if TRACE {
            let enemy_virt_mobility = self.enemy_virt_mobility;
            trace_update!(t, Safety, (piece, enemy_virt_mobility), self.color, kz_attacks);

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
    fn pawn_score<const TRACE: bool>(&self, pawns: Bitboard, color: Color, t: &mut Trace) -> ScoreTuple {
        let piece = Piece::PAWN;
        let pawn_attacks = attacks::pawn_setwise(pawns, color);
        let kz_attacks = self.enemy_king_zone.intersection(pawn_attacks).popcount() as i32;
        let attack_weight = KING_ZONE_ATTACKS[piece.as_index()][self.enemy_virt_mobility];

        if TRACE {
            let enemy_virt_mobility = self.enemy_virt_mobility;
            trace_update!(t, Safety, (piece, enemy_virt_mobility), self.color, kz_attacks);

            trace_threat_update!(t, PAWN_THREAT_ON_KNIGHT, self.color, pawn_attacks, self.enemy_knights);
            trace_threat_update!(t, PAWN_THREAT_ON_BISHOP, self.color, pawn_attacks, self.enemy_bishops);
            trace_threat_update!(t, PAWN_THREAT_ON_ROOK, self.color, pawn_attacks, self.enemy_rooks);
            trace_threat_update!(t, PAWN_THREAT_ON_QUEEN, self.color, pawn_attacks, self.enemy_queens);
        }

        attack_weight.mult(kz_attacks)
            + PAWN_THREAT_ON_KNIGHT.mult((pawn_attacks & self.enemy_knights).popcount() as i32)
            + PAWN_THREAT_ON_BISHOP.mult((pawn_attacks & self.enemy_bishops).popcount() as i32)
            + PAWN_THREAT_ON_ROOK.mult((pawn_attacks & self.enemy_rooks).popcount() as i32)
            + PAWN_THREAT_ON_QUEEN.mult((pawn_attacks & self.enemy_queens).popcount() as i32)
    }

    fn piece_loop<const PIECE: u8, const TRACE: bool>(
        &self,
        board: &Board,
        mut piece_bb: Bitboard,
        t: &mut Trace,
    ) -> ScoreTuple {
        let mut score = ScoreTuple::new(0, 0);
        bitloop!(|sq| piece_bb, {
            score += self.single_score::<PIECE, TRACE>(board, sq, t);
        });
        score
    }
}

pub fn one_sided_eval<const TRACE: bool>(
    board: &Board,
    color: Color,
    t: &mut Trace,
) -> ScoreTuple {
    let knights = board.piece_bb(Piece::KNIGHT, color);
    let bishops = board.piece_bb(Piece::BISHOP, color);
    let rooks = board.piece_bb(Piece::ROOK, color);
    let queens = board.piece_bb(Piece::QUEEN, color);
    let pawns = board.piece_bb(Piece::PAWN, color);

    let looper = LoopEvaluator::new(board, color);
    looper.piece_loop::<{ ConstPiece::KNIGHT }, TRACE>(board, knights, t)
        + looper.piece_loop::<{ ConstPiece::BISHOP }, TRACE>(board, bishops, t)
        + looper.piece_loop::<{ ConstPiece::ROOK }, TRACE>(board, rooks, t)
        + looper.piece_loop::<{ ConstPiece::QUEEN }, TRACE>(board, queens, t)
        + looper.pawn_score::<TRACE>(pawns, color, t)
}

pub fn mobility_threats_safety<const TRACE: bool>(
    board: &Board,
    us: Color,
    them: Color,
    t: &mut Trace,
) -> ScoreTuple {
    one_sided_eval(board, us, t) - one_sided_eval(board, them, t)
}

#[cfg(test)]
mod tests {
    use crate::{
        board::attacks,
        board::board_representation::{Board, Color, Square},
        eval::piece_loop_eval::forward_mobility,
    };

    use super::enemy_virtual_mobility;

    #[test]
    fn virtual_mobility_test() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let w_enemy_virt_mobility = enemy_virtual_mobility(&board, Color::White);
        let b_enemy_virt_mobility = enemy_virtual_mobility(&board, Color::Black);

        assert_eq!(w_enemy_virt_mobility, 5);
        assert_eq!(b_enemy_virt_mobility, 2);
    }

    #[test]
    fn forward_mobility_test() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let sq = Square::D3;
        let moves = attacks::queen(sq, board.occupied());
        let f_mobility = forward_mobility(moves, sq, Color::Black);

        assert_eq!(f_mobility, 5);
    }
}
