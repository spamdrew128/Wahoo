use crate::board_representation::{Board, START_FEN};

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
}

pub struct UciHandler {
    board: Board,
}

impl UciHandler {
    pub fn new() -> Self {
        Self {
            board: Board::from_fen(START_FEN),
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
                        i += 1;
                        START_FEN.to_owned()
                    } else {
                        i = 7;
                        format!("{} {} {} {} {} {}", message[1], message[2], message[3], message[4], message[5], message[6])
                    };

                    let mut mv_vec: Vec<String> = vec![];
                    for mv_str in &message[i..] {
                        mv_vec.push((*mv_str).to_string());
                    }
                    self.process_command(UciCommand::Position(fen, mv_vec));
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
                println!("uci ok");
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::UciNewGame => unimplemented!(),
            UciCommand::Position(fen, moves) => {
                self.board = Board::from_fen(fen.as_str());
                
            },
        }
    }
}
