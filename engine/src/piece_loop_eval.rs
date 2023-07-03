use crate::{
    attacks, bitloop,
    board_representation::{Bitboard, Board, Color, Piece, Square, NUM_COLORS, NUM_SQUARES},
    eval_constants::{
        BISHOP_MOBILITY, KING_ZONE_ATTACKS, KNIGHT_MOBILITY, QUEEN_MOBILITY, ROOK_MOBILITY,
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

#[allow(clippy::cast_possible_wrap)]
fn single_score<const PIECE: u8>(
    board: &Board,
    sq: Square,
    availible: Bitboard,
    enemy_king_zone: Bitboard,
    enemy_virt_mobility: usize,
) -> ScoreTuple {
    match PIECE {
        PieceNum::KNIGHT => {
            let moves = attacks::knight(sq) & availible;
            let kz_attacks = moves & enemy_king_zone;
            let attack_weight = KING_ZONE_ATTACKS[Piece::KNIGHT.as_index()][enemy_virt_mobility];
            KNIGHT_MOBILITY[moves.popcount() as usize]
                + attack_weight.mult(kz_attacks.popcount() as i32)
        }
        PieceNum::BISHOP => {
            let moves = attacks::bishop(sq, board.occupied()) & availible;
            let kz_attacks = moves & enemy_king_zone;
            let attack_weight = KING_ZONE_ATTACKS[Piece::BISHOP.as_index()][enemy_virt_mobility];
            BISHOP_MOBILITY[moves.popcount() as usize]
                + attack_weight.mult(kz_attacks.popcount() as i32)
        }
        PieceNum::ROOK => {
            let moves = attacks::rook(sq, board.occupied()) & availible;
            let kz_attacks = moves & enemy_king_zone;
            let attack_weight = KING_ZONE_ATTACKS[Piece::ROOK.as_index()][enemy_virt_mobility];
            ROOK_MOBILITY[moves.popcount() as usize]
                + attack_weight.mult(kz_attacks.popcount() as i32)
        }
        PieceNum::QUEEN => {
            let moves = attacks::queen(sq, board.occupied()) & availible;
            let kz_attacks = moves & enemy_king_zone;
            let attack_weight = KING_ZONE_ATTACKS[Piece::QUEEN.as_index()][enemy_virt_mobility];
            QUEEN_MOBILITY[moves.popcount() as usize]
                + attack_weight.mult(kz_attacks.popcount() as i32)
        }
        _ => ScoreTuple::new(0, 0),
    }
}

#[allow(clippy::cast_possible_wrap)]
fn pawn_score(
    pawns: Bitboard,
    color: Color,
    enemy_king_zone: Bitboard,
    enemy_virt_mobility: usize,
) -> ScoreTuple {
    let pawn_attacks = attacks::pawn_setwise(pawns, color);
    let kz_attacks = pawn_attacks & enemy_king_zone;
    let attack_weight = KING_ZONE_ATTACKS[Piece::PAWN.as_index()][enemy_virt_mobility];

    attack_weight.mult(kz_attacks.popcount() as i32)
}

fn piece_loop<const PIECE: u8>(
    board: &Board,
    availible: Bitboard,
    mut piece_bb: Bitboard,
    enemy_king_zone: Bitboard,
    enemy_virt_mobility: usize,
) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);
    bitloop!(|sq|, piece_bb, {
        score += single_score::<PIECE>(board, sq, availible, enemy_king_zone, enemy_virt_mobility);
    });
    score
}

pub fn mobility(board: &Board, color: Color) -> ScoreTuple {
    let knights = board.piece_bb(Piece::KNIGHT, color);
    let bishops = board.piece_bb(Piece::BISHOP, color);
    let rooks = board.piece_bb(Piece::ROOK, color);
    let queens = board.piece_bb(Piece::QUEEN, color);
    let pawns = board.piece_bb(Piece::PAWN, color);

    let availible = availible(board, color);
    let enemy_king_zone = enemy_king_zone(board, color);
    let enemy_virt_mobility = enemy_virtual_mobility(board, color);

    piece_loop::<{ PieceNum::KNIGHT }>(
        board,
        availible,
        knights,
        enemy_king_zone,
        enemy_virt_mobility,
    ) + piece_loop::<{ PieceNum::BISHOP }>(
        board,
        availible,
        bishops,
        enemy_king_zone,
        enemy_virt_mobility,
    ) + piece_loop::<{ PieceNum::ROOK }>(
        board,
        availible,
        rooks,
        enemy_king_zone,
        enemy_virt_mobility,
    ) + piece_loop::<{ PieceNum::QUEEN }>(
        board,
        availible,
        queens,
        enemy_king_zone,
        enemy_virt_mobility,
    ) + pawn_score(pawns, color, enemy_king_zone, enemy_virt_mobility)
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
