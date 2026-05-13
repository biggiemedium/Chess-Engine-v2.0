use crate::board::board::Board;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

pub struct Engine {
    movegen: MoveGen,
}

impl Engine {
    pub fn new() -> Engine {
        Self {
            movegen: MoveGen::new(),
        }
    }

    pub fn find_best_move(&self, board: &Board, white_to_move: bool) -> Option<Move> {

        // put all possible moves in a list -> then lets start filtering
        let mut moves = Vec::new();
        self.movegen.generate_moves(board, white_to_move, &mut moves);

        if moves.is_empty() {
            return None;
        }

        let mut best_move = moves[0];
        let mut best_score = i32::MIN; // signed integer -> (-) = black | (+) = white


        // Some value of type (T)
        Some(best_move)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}