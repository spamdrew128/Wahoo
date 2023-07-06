use crate::{
    attacks, bitloop,
    board_representation::{Bitboard, Board, Color, Piece, Square, NUM_COLORS, NUM_SQUARES},
    eval_constants::{
        BISHOP_MOBILITY, BISHOP_THREAT_ON_KNIGHT, BISHOP_THREAT_ON_QUEEN, BISHOP_THREAT_ON_ROOK,
        KING_ZONE_ATTACKS, KNIGHT_MOBILITY, KNIGHT_THREAT_ON_BISHOP, KNIGHT_THREAT_ON_QUEEN,
        KNIGHT_THREAT_ON_ROOK, PAWN_THREAT_ON_BISHOP, PAWN_THREAT_ON_KNIGHT, PAWN_THREAT_ON_QUEEN,
        PAWN_THREAT_ON_ROOK, QUEEN_MOBILITY, ROOK_MOBILITY, ROOK_THREAT_ON_QUEEN,
    },
    evaluation::ScoreTuple,
};

const fn enemy_king_zones_init() -> [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] {
    let mut enemy_king_zones = [[Bitboard::new(0); NUM_SQUARES as usize]; NUM_COLORS as usize];
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

        enemy_king_zones[Color::White.as_index()][sq.as_index()] = black_zone;
        enemy_king_zones[Color::Black.as_index()][sq.as_index()] = white_zone;
        i += 1;
    }

    enemy_king_zones
}

const ENEMY_KING_ZONES: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] =
    enemy_king_zones_init();

pub const fn enemy_king_zone(board: &Board, attacking_color: Color) -> Bitboard {
    let enemy_king_sq = board.color_king_sq(attacking_color.flip());
    ENEMY_KING_ZONES[attacking_color.as_index()][enemy_king_sq.as_index()]
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
}

struct PieceNum;
impl PieceNum {
    const KNIGHT: u8 = 0;
    const BISHOP: u8 = 1;
    const ROOK: u8 = 2;
    const QUEEN: u8 = 3;
    const PAWN: u8 = 4;
    const KING: u8 = 5;
}

struct LoopEvaluator {
    availible: Bitboard,
    enemy_king_zone: Bitboard,
    enemy_virt_mobility: usize,
    enemy_knights: Bitboard,
    enemy_bishops: Bitboard,
    enemy_rooks: Bitboard,
    enemy_queens: Bitboard,
}

macro_rules! score_func {
    ($fn_name:ident, $piece:ident, $mobility_tb:ident, ($sq:ident $(, $occ:ident)?), $attack_fn:expr, $(($threat_constant:ident, $threatened_piece:ident, $target:ident)),*) => {
        #[inline(always)]
        fn $fn_name(board: &Board, availible: Bitboard, enemy_kz: Bitboard, enemy_virt_mobility: usize, color: Color) -> ScoreTuple {
            let mut score = ScoreTuple::new(0, 0);
            let mut pieces = board.piece_bb(Piece::$piece, color);
            $(let $occ = board.occupied();)?

            $(
                let $target = board.piece_bb(Piece::$threatened_piece, color.flip());
            )*

            bitloop!(|$sq|, pieces, {
                let attacks = $attack_fn;
                let moves = attacks & availible;

                score += $mobility_tb[moves.popcount() as usize];

                let kz_attacks = moves & enemy_kz;
                let attack_weight =
                    KING_ZONE_ATTACKS[Piece::$piece.as_index()][enemy_virt_mobility];
                score += attack_weight.mult(kz_attacks.popcount() as i32);

                $(
                    score += $threat_constant.mult((attacks & $target).popcount() as i32);
                )*
            });
            score
        }
    };
}

score_func!(
    knight_score,
    KNIGHT,
    KNIGHT_MOBILITY,
    (sq),
    { attacks::knight(sq) },
    (KNIGHT_THREAT_ON_BISHOP, BISHOP, k),
    (KNIGHT_THREAT_ON_ROOK, ROOK, r),
    (KNIGHT_THREAT_ON_QUEEN, QUEEN, q)
);
score_func!(
    bishop_score,
    BISHOP,
    BISHOP_MOBILITY,
    (sq, occ),
    { attacks::bishop(sq, occ) },
    (BISHOP_THREAT_ON_KNIGHT, KNIGHT, k),
    (BISHOP_THREAT_ON_ROOK, ROOK, r),
    (BISHOP_THREAT_ON_QUEEN, QUEEN, q)
);
score_func!(
    rook_score,
    ROOK,
    ROOK_MOBILITY,
    (sq, occ),
    { attacks::rook(sq, occ) },
    (ROOK_THREAT_ON_QUEEN, QUEEN, q)
);
score_func!(queen_score, QUEEN, QUEEN_MOBILITY, (sq, occ), {
    attacks::queen(sq, occ)
},);

#[allow(clippy::cast_possible_wrap)]
fn pawn_score(
    board: &Board,
    enemy_king_zone: Bitboard,
    enemy_virt_mobility: usize,
    color: Color,
) -> ScoreTuple {
    let pawns = board.piece_bb(Piece::PAWN, color);
    let pawn_attacks = attacks::pawn_setwise(pawns, color);
    let kz_attacks = pawn_attacks & enemy_king_zone;
    let attack_weight = KING_ZONE_ATTACKS[Piece::PAWN.as_index()][enemy_virt_mobility];

    let opp_color = color.flip();
    attack_weight.mult(kz_attacks.popcount() as i32)
        + PAWN_THREAT_ON_KNIGHT
            .mult((pawn_attacks & board.piece_bb(Piece::KNIGHT, opp_color)).popcount() as i32)
        + PAWN_THREAT_ON_BISHOP
            .mult((pawn_attacks & board.piece_bb(Piece::BISHOP, opp_color)).popcount() as i32)
        + PAWN_THREAT_ON_ROOK
            .mult((pawn_attacks & board.piece_bb(Piece::ROOK, opp_color)).popcount() as i32)
        + PAWN_THREAT_ON_QUEEN
            .mult((pawn_attacks & board.piece_bb(Piece::QUEEN, opp_color)).popcount() as i32)
}

pub fn mobility_threats_safety(board: &Board, color: Color) -> ScoreTuple {
    let availible = availible(board, color);
    let virt_mobility = enemy_virtual_mobility(board, color);
    let kz = enemy_king_zone(board, color);

    knight_score(board, availible, kz, virt_mobility, color)
        + bishop_score(board, availible, kz, virt_mobility, color)
        + rook_score(board, availible, kz, virt_mobility, color)
        + queen_score(board, availible, kz, virt_mobility, color)
        + pawn_score(board, kz, virt_mobility, color)
}

#[cfg(test)]
mod tests {
    use crate::board_representation::{Board, Color};

    use super::enemy_virtual_mobility;

    #[test]
    fn virtual_mobility() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let w_enemy_virt_mobility = enemy_virtual_mobility(&board, Color::White);
        let b_enemy_virt_mobility = enemy_virtual_mobility(&board, Color::Black);

        assert_eq!(w_enemy_virt_mobility, 5);
        assert_eq!(b_enemy_virt_mobility, 2);
    }
}
