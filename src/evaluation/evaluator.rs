

use crate::board::board::Board;
use crate::evaluation::{materials, pst};

pub struct Evaluator;

impl Evaluator {

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        // Material evaluation
        score += materials::MaterialEvaluator::evaluate(board);

        // Piece-square table evaluation
        score += pst::PieceSquareTableEvaluator::evaluate(board);

        score
    }
}