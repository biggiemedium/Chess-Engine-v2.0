use crate::board::board::Board;
use crate::board::debug::Debug;
use std::io::{self, Write};
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

pub struct Game {
    board: Board,
    movegen: MoveGen,
    white_to_move: bool,
    move_history: Vec<(Move, Option<crate::board::board::PieceType>)>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            movegen: MoveGen::new(),
            white_to_move: true,
            move_history: Vec::new(),
        }
    }

    pub fn play(&mut self) {
        println!("Chess Engine - Interactive Mode");
        println!("Commands:");
        println!("  - Enter move in UCI format (e.g., e2e4, e7e8q for promotion)");
        println!("  - 'moves' - show all legal moves");
        println!("  - 'undo' - undo last move");
        println!("  - 'quit' - exit game");
        println!();

        loop {
            Debug::print_board(&self.board);
            println!("\n{} to move", if self.white_to_move { "White" } else { "Black" });

            let mut moves = Vec::new();
            self.movegen.generate_moves(&self.board, self.white_to_move, &mut moves);

            if moves.is_empty() {
                println!("No legal moves! Game over.");
                break;
            }

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "quit" | "exit" => break,
                "moves" => {
                    self.show_moves(&moves);
                    continue;
                }
                "undo" => {
                    if let Some((mv, captured)) = self.move_history.pop() {
                        self.board.unmake_move(&mv, !self.white_to_move, captured);
                        self.white_to_move = !self.white_to_move;
                        println!("Move undone");
                    } else {
                        println!("No moves to undo");
                    }
                    continue;
                }
                _ => {
                    if let Some(mv) = self.parse_move(input, &moves) {
                        let captured = self.board.piece_at(mv.to).map(|(pt, _)| pt);
                        self.board.make_move(&mv, self.white_to_move);
                        self.move_history.push((mv, captured));
                        self.white_to_move = !self.white_to_move;
                    } else {
                        println!("Invalid move! Try again or type 'moves' to see legal moves.");
                    }
                }
            }
        }
    }

    fn parse_move(&self, input: &str, legal_moves: &[Move]) -> Option<Move> {
        if input.len() < 4 {
            return None;
        }

        let from_file = (input.as_bytes()[0] as i8 - b'a' as i8) as u8;
        let from_rank = (input.as_bytes()[1] as i8 - b'1' as i8) as u8;
        let to_file = (input.as_bytes()[2] as i8 - b'a' as i8) as u8;
        let to_rank = (input.as_bytes()[3] as i8 - b'1' as i8) as u8;

        if from_file > 7 || from_rank > 7 || to_file > 7 || to_rank > 7 {
            return None;
        }

        let from = from_rank * 8 + from_file;
        let to = to_rank * 8 + to_file;

        // Find matching legal move
        for mv in legal_moves {
            if mv.from == from && mv.to == to {
                // Handle promotion
                if input.len() == 5 {
                    let promo = input.as_bytes()[4];
                    let is_capture = mv.is_capture();
                    let flags = match promo {
                        b'n' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_KNIGHT } else { crate::movegen::r#move::MoveFlags::PROMOTION_KNIGHT },
                        b'b' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_BISHOP } else { crate::movegen::r#move::MoveFlags::PROMOTION_BISHOP },
                        b'r' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_ROOK } else { crate::movegen::r#move::MoveFlags::PROMOTION_ROOK },
                        b'q' => if is_capture { crate::movegen::r#move::MoveFlags::PROMOTION_CAPTURE_QUEEN } else { crate::movegen::r#move::MoveFlags::PROMOTION_QUEEN },
                        _ => return None,
                    };
                    if mv.flags == flags {
                        return Some(*mv);
                    }
                } else if !mv.is_promotion() {
                    return Some(*mv);
                }
            }
        }

        None
    }

    fn show_moves(&self, moves: &[Move]) {
        println!("\nLegal moves:");
        for mv in moves {
            print!("{} ", self.move_to_string(mv));
        }
        println!("\n");
    }

    fn move_to_string(&self, mv: &Move) -> String {
        let from_file = (mv.from % 8) as u8;
        let from_rank = (mv.from / 8) as u8;
        let to_file = (mv.to % 8) as u8;
        let to_rank = (mv.to / 8) as u8;

        let mut s = format!(
            "{}{}{}{}",
            (b'a' + from_file) as char,
            (b'1' + from_rank) as char,
            (b'a' + to_file) as char,
            (b'1' + to_rank) as char
        );

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
            s.push(promo);
        }

        s
    }
}