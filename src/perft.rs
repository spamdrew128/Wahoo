use crate::board_representation::Board;
use crate::movegen::MoveGenerator;

fn perft(board: Board, depth: u16, count: &mut u64) {
    if depth == 0 {
        *count += 1;
        return;
    }

    let mut generator = MoveGenerator::new();

    while let Some(mv) = generator.next(&board) {
        if let Some(new_board) = board.try_play_move(mv) {
            perft(new_board, depth - 1, count);
        }
    }
}

pub fn split_perft(fen: &str, depth: u16) {
    let board = Board::from_fen(fen);
    let mut generator = MoveGenerator::new();

    while let Some(mv) = generator.next(&board) {
        if let Some(new_board) = board.try_play_move(mv) {
            let mut count = 0;
            perft(new_board, depth - 1, &mut count);
            println!("{} - {}", mv.as_string(), count);
        }
    }
}
