
use crate::board::board::Board;
use crate::evaluation::{materials, pst, kingsafteyevalution};
use crate::evaluation::pawnevaluation::PawnStructureEvaluator;
use crate::evaluation::tempoeval::TempoEvaluator;
use crate::evaluation::trappedpiece::{MobilityEvaluator, TrappedPieceEvaluator};
use crate::movegen::movegeneration::MoveGen;

pub struct Evaluator;

// TODO
/**
Basic Evaluation Features
- Evaluation of Pieces (kinda done ?) - https://www.chessprogramming.org/Evaluation_of_Pieces
- Evaluation Patterns - https://www.chessprogramming.org/Evaluation_Patterns
- Center Control - https://www.chessprogramming.org/Center_Control
- Connectivity - https://www.chessprogramming.org/Connectivity
- Space - https://www.chessprogramming.org/Space
*/
impl Evaluator {

    pub fn evaluate(board: &Board, white_to_move: bool) -> i32 {
        Self::evaluate_with_movegen(board, white_to_move, None)
    }

    pub fn evaluate_with_movegen(board: &Board, white_to_move: bool, movegen: Option<&MoveGen>) -> i32 {
        let mut score = 0;

        // material evaluation
        score += materials::MaterialEvaluator::evaluate(board);

        // piece square table evaluation
        score += pst::PieceSquareTableEvaluator::evaluate(board);

        // king safety
        score += kingsafteyevalution::KingSafteyEvaluation::evaluate(board);

        // pawn structure
        score += PawnStructureEvaluator::evaluate(board);
        
        // Tempo
        score += TempoEvaluator::evaluate(board, white_to_move);

        // If movegen is available -> evaluate mobility and trapped pieces
        if let Some(mg) = movegen {
            score += TrappedPieceEvaluator::evaluate(board, mg);
            score += MobilityEvaluator::evaluate(board, mg);
        }

        if white_to_move {
            score
        } else {
            -score
        }
    }
}