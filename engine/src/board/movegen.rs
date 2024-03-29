use super::attacks;
use super::board_representation::{Bitboard, Board, Piece, Square, NUM_PIECES};
use super::chess_move::MAX_MOVECOUNT;
use super::chess_move::{Flag, Move};
use crate::bitloop;
use crate::search::history_table::History;
use crate::tuple_constants_enum;

macro_rules! into_moves {
    (|$from:ident|, $piece_bb:ident, |$to:ident|, $moves_bb:expr, $add_move:expr) => {{
        bitloop!(|$from| $piece_bb, {
            let mut moves: Bitboard = $moves_bb;
            bitloop!(|$to| moves, { $add_move });
        });
    }};
}

const MVV_LVA: [[i16; (NUM_PIECES + 1) as usize]; (NUM_PIECES + 1) as usize] = {
    // knight, bishop, rook, queen, pawn, king, none (for en passant)
    let scores: [i16; (NUM_PIECES + 1) as usize] = [3, 4, 5, 9, 1, 0, 1];
    let mut result: [[i16; (NUM_PIECES + 1) as usize]; (NUM_PIECES + 1) as usize] =
        [[0; (NUM_PIECES + 1) as usize]; (NUM_PIECES + 1) as usize];

    let mut a = 0;
    while a < (NUM_PIECES + 1) as usize {
        let mut v = 0;
        while v < (NUM_PIECES + 1) as usize {
            result[a][v] = scores[v] - scores[a];
            v += 1;
        }
        a += 1;
    }

    result
};

const fn mvv_lva(attacker: Piece, victim: Piece) -> i16 {
    MVV_LVA[attacker.as_index()][victim.as_index()]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MoveStage(u8);

impl MoveStage {
    #[rustfmt::skip]
    tuple_constants_enum!(Self,
        START,
        TT,
        CAPTURE,
        KILLER,
        QUIET_AND_BAD_CAP
    );

    const fn new(data: u8) -> Self {
        Self(data)
    }

    fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Copy, Clone)]
struct MoveElement {
    mv: Move,
    score: i16,
}

impl MoveElement {
    const fn new() -> Self {
        Self {
            mv: Move::nullmove(),
            score: 0,
        }
    }
}

pub struct MoveGenerator {
    stage: MoveStage,
    movelist: [MoveElement; MAX_MOVECOUNT],
    limit: usize,
    index: usize,
    bad_captures: usize,
}

impl MoveGenerator {
    pub const fn new() -> Self {
        Self {
            stage: MoveStage::START,
            movelist: [MoveElement::new(); MAX_MOVECOUNT],
            limit: 0,
            index: 0,
            bad_captures: 0,
        }
    }

    const fn stage_complete(&self) -> bool {
        self.index >= self.limit
    }

    fn advance_stage(&mut self) {
        self.stage.increment();
        self.index = self.limit;
    }

    fn add_move(&mut self, mv: Move, repeats: &[Move]) {
        if repeats.contains(&mv) {
            return;
        }
        self.movelist[self.limit].mv = mv;
        self.limit += 1;
    }

    fn pick_move(&mut self) -> Move {
        let mut best_index = self.index;
        let mut best_score = self.movelist[self.index].score;
        for i in (self.index + 1)..self.limit {
            let score = self.movelist[i].score;
            if score > best_score {
                best_score = score;
                best_index = i;
            }
        }

        let mv = self.movelist[best_index].mv;
        self.movelist.swap(self.index, best_index);
        self.index += 1;
        mv
    }

    fn generic_movegen(&mut self, board: &Board, filter: Bitboard, flag: Flag, repeats: &[Move]) {
        let color = board.color_to_move;
        let occupied = board.occupied();

        let mut knights = board.piece_bb(Piece::KNIGHT, color);
        into_moves!(|from|, knights, |to|, attacks::knight(from).intersection(filter), {
            self.add_move(Move::new(to, from, flag), repeats);
        });

        let mut bishops = board.piece_bb(Piece::BISHOP, color);
        into_moves!(|from|, bishops, |to|, attacks::bishop(from, occupied).intersection(filter), {
            self.add_move(Move::new(to, from, flag), repeats);
        });

        let mut rooks = board.piece_bb(Piece::ROOK, color);
        into_moves!(|from|, rooks, |to|, attacks::rook(from, occupied).intersection(filter),{
            self.add_move(Move::new(to, from, flag), repeats);
        });

        let mut queens = board.piece_bb(Piece::QUEEN, color);
        into_moves!(|from|, queens, |to|, attacks::queen(from, occupied).intersection(filter), {
            self.add_move(Move::new(to, from, flag), repeats);
        });

        let mut king = board.piece_bb(Piece::KING, color);
        into_moves!(|from|, king, |to|, attacks::king(from).intersection(filter), {
            self.add_move(Move::new(to, from, flag), repeats);
        });
    }

    fn generate_captures(&mut self, board: &Board, repeats: &[Move]) {
        let color = board.color_to_move;
        let them = board.them();

        let pawns = board.piece_bb(Piece::PAWN, color);
        let mut promoting_pawns = board.promotable_pawns();
        let mut normal_pawns = pawns.without(promoting_pawns);

        into_moves!(|from|, promoting_pawns, |to|, attacks::pawn(from, color).intersection(them), {
            self.add_move(Move::new(to, from, Flag::QUEEN_CAPTURE_PROMO), repeats);
            self.add_move(Move::new(to, from, Flag::KNIGHT_CAPTURE_PROMO), repeats);
            self.add_move(Move::new(to, from, Flag::ROOK_CAPTURE_PROMO), repeats);
            self.add_move(Move::new(to, from, Flag::BISHOP_CAPTURE_PROMO), repeats);
        });

        into_moves!(|from|, normal_pawns, |to|, attacks::pawn(from, color).intersection(them), {
            self.add_move(Move::new(to, from, Flag::CAPTURE), repeats);
        });

        if let Some(to) = board.ep_sq {
            let mut attackers = attacks::pawn(to, color.flip()).intersection(pawns);
            bitloop!(|from| attackers, {
                self.add_move(Move::new(to, from, Flag::EP), repeats);
            });
        }

        self.generic_movegen(board, them, Flag::CAPTURE, repeats);
    }

    fn generate_quiets(&mut self, board: &Board, repeats: &[Move]) {
        let color = board.color_to_move;
        let empty = board.empty();

        let pawns = board.piece_bb(Piece::PAWN, color);
        let promotable_pawns = board.promotable_pawns();

        let mut promotions = attacks::pawn_single_push(promotable_pawns, empty, color);
        let mut single_pushs =
            attacks::pawn_single_push(pawns.without(promotable_pawns), empty, color);
        let mut double_pushs = attacks::pawn_double_push(single_pushs, empty, color);

        bitloop!(|to| promotions, {
            let from = to.row_swap();
            self.add_move(Move::new(to, from, Flag::QUEEN_PROMO), repeats);
            self.add_move(Move::new(to, from, Flag::KNIGHT_PROMO), repeats);
            self.add_move(Move::new(to, from, Flag::ROOK_PROMO), repeats);
            self.add_move(Move::new(to, from, Flag::BISHOP_PROMO), repeats);
        });

        bitloop!(|to| single_pushs, {
            let from = to.retreat(1, color);
            self.add_move(Move::new(to, from, Flag::NONE), repeats);
        });

        bitloop!(|to| double_pushs, {
            let from = to.double_push_sq();
            self.add_move(Move::new(to, from, Flag::DOUBLE_PUSH), repeats);
        });

        if board.castle_rights.can_ks_castle(board) {
            self.add_move(Move::new_ks_castle(board.king_sq()), repeats);
        }

        if board.castle_rights.can_qs_castle(board) {
            self.add_move(Move::new_qs_castle(board.king_sq()), repeats);
        }

        self.generic_movegen(board, empty, Flag::NONE, repeats);
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn score_captures(&mut self, board: &Board) {
        let mut start = self.index as i32;
        let mut end = self.limit as i32 - 1;

        while start <= end {
            let i = start as usize;
            let mv = self.movelist[i].mv;

            let attacker = board.piece_on_sq(mv.from());
            let victim = board.piece_on_sq(mv.to());
            self.movelist[i].score = mvv_lva(attacker, victim);

            if board.see(mv, attacker, victim, 0) {
                // good capture
                start += 1;
            } else {
                // bad capture
                self.bad_captures += 1;
                self.movelist.swap(i, end as usize);
                end -= 1;
            }
        }

        self.limit -= self.bad_captures; // dont include bad captures in this stage
    }

    fn score_quiets(&mut self, board: &Board, history: &History) {
        for elem in self
            .movelist
            .iter_mut()
            .skip(self.index)
            .take(self.limit - self.index)
        {
            debug_assert!(elem.mv.is_quiet());
            elem.score = history.score(board, elem.mv) as i16;
        }
    }

    pub fn next<const FULL_MOVEGEN: bool>(
        &mut self,
        board: &Board,
        history: &History,
        killer: Move,
        tt_move: Move,
    ) -> Option<Move> {
        while self.stage_complete() {
            self.advance_stage();

            match self.stage {
                MoveStage::TT => {
                    if tt_move.is_pseudolegal(board) {
                        return Some(tt_move);
                    }
                }
                MoveStage::CAPTURE => {
                    if tt_move.is_capture() {
                        self.generate_captures(board, &[tt_move]);
                    } else {
                        self.generate_captures(board, &[]);
                    };
                    self.score_captures(board);
                }
                MoveStage::KILLER => {
                    if !FULL_MOVEGEN {
                        return None;
                    }

                    if killer.is_pseudolegal(board) {
                        return Some(killer);
                    }
                }
                MoveStage::QUIET_AND_BAD_CAP => {
                    self.limit += self.bad_captures;
                    self.index = self.limit;

                    if !tt_move.is_null() && tt_move.is_quiet() {
                        self.generate_quiets(board, &[tt_move, killer]);
                    } else {
                        self.generate_quiets(board, &[killer]);
                    };

                    self.score_quiets(board, history);
                    self.index -= self.bad_captures;
                }
                _ => return None,
            }
        }

        Some(self.pick_move())
    }

    pub fn simple_next<const FULL_MOVEGEN: bool>(&mut self, board: &Board) -> Option<Move> {
        self.next::<FULL_MOVEGEN>(board, &History::new(), Move::nullmove(), Move::nullmove())
    }

    pub fn first_legal_move(board: &Board) -> Option<Move> {
        let mut generator = Self::new();
        while let Some(mv) = generator.simple_next::<true>(board) {
            let mut new_board = board.clone();
            if new_board.simple_try_play_move(mv) {
                return Some(mv);
            }
        }

        None
    }

    pub fn no_legal_moves(board: &Board) -> bool {
        Self::first_legal_move(board).is_none()
    }

    pub const fn stage(&self) -> MoveStage {
        self.stage
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_captures() {
        use super::*;

        let board = Board::from_fen("1n4K1/P2k2b1/4r1n1/PpPB4/5N2/bRq1r3/3P4/2Q5 w - b6 0 2");
        let mut counts = [0; NUM_PIECES as usize];
        let mut promo_count = 0;
        let mut ep_count = 0;

        let mut generator = MoveGenerator::new();
        while let Some(mv) = generator.simple_next::<false>(&board) {
            let piece = board.piece_on_sq(mv.from());
            counts[piece.as_index()] += 1;

            if mv.is_promo() {
                promo_count += 1;
            }

            if mv.flag() == Flag::EP {
                ep_count += 1;
            }
        }

        assert_eq!(counts[Piece::PAWN.as_index()], 8);
        assert_eq!(counts[Piece::BISHOP.as_index()], 1);
        assert_eq!(counts[Piece::ROOK.as_index()], 3);
        assert_eq!(counts[Piece::QUEEN.as_index()], 2);
        assert_eq!(counts[Piece::KNIGHT.as_index()], 2);
        assert_eq!(counts[Piece::KING.as_index()], 1);
        assert_eq!(promo_count, 4);
        assert_eq!(ep_count, 2);
    }

    #[test]
    fn generates_quiets() {
        use super::*;

        let board = Board::from_fen(
            "r3k2r/pPppqpb1/bn2pnp1/3PN3/1p2P3/1nN2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0",
        );
        let mut counts = [0; NUM_PIECES as usize];
        let mut promo_count = 0;
        let mut castle_count = 0;

        let mut generator = MoveGenerator::new();
        while let Some(mv) = generator.simple_next::<true>(&board) {
            if mv.is_quiet() {
                let piece = board.piece_on_sq(mv.from());
                counts[piece.as_index()] += 1;

                if mv.is_promo() {
                    promo_count += 1;
                }

                if mv.flag() == Flag::KS_CASTLE || mv.flag() == Flag::QS_CASTLE {
                    castle_count += 1;
                }
            }
        }

        assert_eq!(counts[Piece::PAWN.as_index()], 9);
        assert_eq!(counts[Piece::BISHOP.as_index()], 10);
        assert_eq!(counts[Piece::ROOK.as_index()], 5);
        assert_eq!(counts[Piece::QUEEN.as_index()], 7);
        assert_eq!(counts[Piece::KNIGHT.as_index()], 8);
        assert_eq!(counts[Piece::KING.as_index()], 3);
        assert_eq!(promo_count, 4);
        assert_eq!(castle_count, 1);
    }

    #[test]
    fn correct_move_count() {
        use super::*;

        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let expected_count = 48;
        let mut actual = 0;
        let mut list = vec![];

        let mut g = MoveGenerator::new();
        while let Some(mv) = g.simple_next::<true>(&board) {
            actual += 1;
            assert!(!list.contains(&mv), "{} is duplicate", mv.as_string());
            list.push(mv);
        }
        assert_eq!(expected_count, actual);
    }
}
