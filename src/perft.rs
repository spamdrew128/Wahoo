use crate::board_representation::Board;
use crate::movegen::MoveGenerator;

fn perft(board: Board, depth: u8, count: &mut u64) {
    if depth == 0 {
        *count += 1;
    }

    let mut generator = MoveGenerator::new();

    while let Some(mv) = generator.next(&board) {
        if let Some(new_board) = board.try_play_move(mv) {
            perft(new_board, depth - 1, count);
        }
    }
}

fn split_perft(fen: &str, depth: u8) {
    let board = Board::from_fen(fen);
    let mut generator = MoveGenerator::new();

    while let Some(mv) = generator.next(&board) {
        if let Some(new_board) = board.try_play_move(mv) {
            let mut count = 0;
            perft(new_board, depth, &mut count);
            println!("{} - {}", mv.as_string(), count);
        }
    }
}
