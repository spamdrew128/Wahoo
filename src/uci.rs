use crate::{
    board_representation::{Board, START_FEN},
    chess_move::Move,
    search::Searcher,
    time_management::{TimeArgs, TimeManager, Milliseconds},
};

#[derive(Debug, Copy, Clone)]
pub enum ProgramStatus {
    Run,
    Quit,
}

enum UciCommand {
    Uci,
    IsReady,
    UciNewGame,
    Position(String, Vec<String>),
    Go(Vec<String>),
}

pub struct UciHandler {
    board: Board,
    time_manager: TimeManager,
    searcher: Searcher,
}

impl UciHandler {
    pub fn new() -> Self {
        Self {
            board: Board::from_fen(START_FEN),
            time_manager: TimeManager::new(),
            searcher: Searcher::new(),
        }
    }

    pub fn execute_instructions(&mut self) -> ProgramStatus {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("UCI Input Failure");

        let message = buffer.split_whitespace().collect::<Vec<&str>>();
        if let Some(cmd) = message.first() {
            match *cmd {
                "uci" => self.process_command(UciCommand::Uci),
                "isready" => self.process_command(UciCommand::IsReady),
                "position" => {
                    let mut i = 1;
                    let fen = if message[i] == "startpos" {
                        i += 2;
                        START_FEN.to_owned()
                    } else {
                        i = 9;
                        format!(
                            "{} {} {} {} {} {}",
                            message[2], message[3], message[4], message[5], message[6], message[7]
                        )
                    };

                    let mut mv_vec: Vec<String> = vec![];
                    if i < message.len() {
                        for mv_str in &message[i..] {
                            mv_vec.push((*mv_str).to_string());
                        }
                    }

                    self.process_command(UciCommand::Position(fen, mv_vec));
                }
                "go" => {
                    let mut arg_vec: Vec<String> = vec![];
                    for arg in &message[1..] {
                        arg_vec.push((*arg).to_string());
                    }
                    self.process_command(UciCommand::Go(arg_vec));
                }
                "quit" => return ProgramStatus::Quit,
                _ => (),
            }
        }

        ProgramStatus::Run
    }

    fn process_command(&mut self, command: UciCommand) {
        match command {
            UciCommand::Uci => {
                println!("id name Wahoo v0.0.0");
                println!("id author Andrew Hockman");
                println!("uciok");
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::UciNewGame => (),
            UciCommand::Position(fen, move_vec) => {
                let mut new_board = Board::from_fen(fen.as_str());
                for mv_str in move_vec {
                    let mv = Move::from_string(mv_str.as_str(), &new_board);
                    let success = new_board.try_play_move(mv);
                    if !success {
                        return;
                    }
                }
                self.board = new_board;
            }
            UciCommand::Go(arg_vec) => {
                let mut time_args = TimeArgs::default();
                let mut args_iterator = arg_vec.iter();

                while let Some(arg) = args_iterator.next() {
                    match arg.as_str() {
                        "wtime" => {
                            time_args.w_time = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap();
                        }
                        "btime" => {
                            time_args.b_time = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap();
                        }
                        "winc" => {
                            time_args.w_inc = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap();
                        }
                        "binc" => {
                            time_args.b_inc = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap();
                        }
                        "movetime" => {
                            time_args.move_time = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap();
                        }
                        "infinite" => time_args.infinite_mode = true,
                        _ => (),
                    }
                }

                let search_timer = self
                    .time_manager
                    .construct_search_timer(time_args, self.board.color_to_move);

                self.searcher.go(&self.board, search_timer);
            }
        }
    }
}
