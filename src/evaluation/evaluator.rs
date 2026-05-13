
use crate::board::board::Board;
use crate::evaluation::{materials, pst, kingsafteyevalution};

pub struct Evaluator;

// TODO
/**
Basic Evaluation Features
- Pawn Structure - https://www.chessprogramming.org/Pawn_Structure
- Evaluation of Pieces (kinda done ?) - https://www.chessprogramming.org/Evaluation_of_Pieces
- Evaluation Patterns - https://www.chessprogramming.org/Evaluation_Patterns
- Mobility - https://www.chessprogramming.org/Mobility
- Center Control - https://www.chessprogramming.org/Center_Control
- Connectivity - https://www.chessprogramming.org/Connectivity
- Trapped Pieces - https://www.chessprogramming.org/Trapped_Pieces
- Space - https://www.chessprogramming.org/Space
- Tempo - https://www.chessprogramming.org/Tempo
*/
impl Evaluator {

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        // material evaluation
        score += materials::MaterialEvaluator::evaluate(board);

        // piece square table evaluation
        score += pst::PieceSquareTableEvaluator::evaluate(board);

        // king saftey
        score += kingsafteyevalution::KingSafteyEvaluation::evaluate(board);

        score
    }
}