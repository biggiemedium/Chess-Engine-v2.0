use crate::board::board::Board;
use crate::evaluation::evaluator::Evaluator;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;
use crate::search::search;
use crate::search::search::Search;

pub struct Engine {
    movegen: MoveGen,
    search: Search
}

impl Engine {
    pub fn new() -> Engine {
        Self {
            movegen: MoveGen::new(),
            search: Search::new()
        }
    }
    /// Find the best move for the current position
    pub fn find_best_move(&mut self, board: &Board, white_to_move: bool, depth: u8) -> Option<Move> {
        self.search.find_best_move(&self.movegen, board, white_to_move, depth)
    }

    /// Get a static evaluation of the position
    pub fn evaluate(&self, board: &Board) -> i32 {
        Evaluator::evaluate(board)
    }

}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}