/**
https://en.wikipedia.org/wiki/Universal_Chess_Interface
https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

*/

use crate::board::board::Board;
use std::io::{self, BufRead};
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

pub struct Uci {
    board: Board,
    movegen: MoveGen,
    white_to_move: bool,
}

impl Uci {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            movegen: MoveGen::new(),
            white_to_move: true,
        }
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut line = String::new();

        loop {
            line.clear();
            if reader.read_line(&mut line).is_err() {
                break;
            }

            let input = line.trim();
            if input.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = input.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "uci" => self.uci(),
                "isready" => self.isready(),
                "ucinewgame" => self.new_game(),
                "position" => self.position(&tokens[1..]),
                "go" => self.go(),
                "quit" => break,
                "d" => self.display(),
                _ => println!("Unknown command: {}", tokens[0]),
            }
        }
    }

    fn uci(&self) {
        println!("id name ChessEngine2");
        println!("id author biggiemedium");
        println!("uciok");
    }

    fn isready(&self) {
        println!("readyok");
    }

    fn new_game(&mut self) {
        self.board = Board::new();
        self.white_to_move = true;
    }

    fn position(&mut self, tokens: &[&str]) {
        if tokens.is_empty() {
            return;
        }

        let mut idx = 0;

        // Handle starting position
        match tokens[0] {
            "startpos" => {
                self.board = Board::new();
                self.white_to_move = true;
                idx = 1;
            }
            "fen" => {
                // TODO: Implement FEN parsing
                println!("FEN parsing not yet implemented");
                return;
            }
            _ => return,
        }

        // Handle moves
        if idx < tokens.len() && tokens[idx] == "moves" {
            idx += 1;
            while idx < tokens.len() {
                if let Some(mv) = self.parse_uci_move(tokens[idx]) {
                    self.board.make_move(&mv, self.white_to_move);
                    self.white_to_move = !self.white_to_move;
                } else {
                    println!("Invalid move: {}", tokens[idx]);
                    return;
                }
                idx += 1;
            }
        }
    }

    fn parse_uci_move(&self, uci: &str) -> Option<Move> {
        if uci.len() < 4 {
            return None;
        }

        let from_file = (uci.as_bytes()[0] - b'a') as u8;
        let from_rank = (uci.as_bytes()[1] - b'1') as u8;
        let to_file = (uci.as_bytes()[2] - b'a') as u8;
        let to_rank = (uci.as_bytes()[3] - b'1') as u8;

        if from_file > 7 || from_rank > 7 || to_file > 7 || to_rank > 7 {
            return None;
        }

        let from = from_rank * 8 + from_file;
        let to = to_rank * 8 + to_file;

        // Generate all legal moves and find matching one
        let mut moves = Vec::new();
        self.movegen.generate_moves(&self.board, self.white_to_move, &mut moves);

        for mv in moves {
            if mv.from == from && mv.to == to {
                // Handle promotion
                if uci.len() == 5 {
                    let promo = uci.as_bytes()[4];
                    let is_capture = mv.is_capture();
                    let flags = match promo {
                        b'n' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_KNIGHT } else { crate::movegen::r#move::MoveFlags::PROMOTION_KNIGHT },
                        b'b' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_BISHOP } else { crate::movegen::r#move::MoveFlags::PROMOTION_BISHOP },
                        b'r' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_ROOK } else { crate::movegen::r#move::MoveFlags::PROMOTION_ROOK },
                        b'q' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_QUEEN } else { crate::movegen::r#move::MoveFlags::PROMOTION_QUEEN },
                        _ => return None,
                    };
                    if mv.flags == flags {
                        return Some(mv);
                    }
                } else {
                    return Some(mv);
                }
            }
        }

        None
    }

    fn go(&mut self) {
        let mut moves = Vec::new();
        self.movegen.generate_moves(&self.board, self.white_to_move, &mut moves);

        if moves.is_empty() {
            println!("info string No legal moves available");
            return;
        }

        // For now -> just play the first legal move
        let best_move = &moves[0];
        println!("bestmove {}", self.move_to_uci(best_move));
    }

    fn move_to_uci(&self, mv: &Move) -> String {
        let from_file = (mv.from % 8) as u8;
        let from_rank = (mv.from / 8) as u8;
        let to_file = (mv.to % 8) as u8;
        let to_rank = (mv.to / 8) as u8;

        let mut uci = format!(
            "{}{}{}{}",
            (b'a' + from_file) as char,
            (b'1' + from_rank) as char,
            (b'a' + to_file) as char,
            (b'1' + to_rank) as char
        );

        // Add promotion piece
        if mv.is_promotion() {
            let promo = if mv.flags.is_promotion_knight() {
                'n'
            } else if mv.flags.is_promotion_bishop() {
                'b'
            } else if mv.flags.is_promotion_rook() {
                'r'
            } else {
                'q'
            };
            uci.push(promo);
        }

        uci
    }

    fn display(&self) {
        crate::board::debug::Debug::print_board(&self.board);
        println!("Side to move: {}", if self.white_to_move { "White" } else { "Black" });
    }
}