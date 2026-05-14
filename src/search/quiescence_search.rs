
//Most chess programs, at the end of the main search perform a more limited quiescence search
// containing fewer moves

use crate::board::board::Board;
use crate::evaluation::evaluator::Evaluator;
use crate::evaluation::materials::MaterialEvaluator;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

// The purpose of Quiescence search is to only evaluate "quiet" positions,
// or positions where there are no winning tactical moves to be made
pub struct QuiescenceSearch {
    depth: i8, // to prevent search "explosion"
    evaluator: Evaluator,
    move_gen: MoveGen
}

impl QuiescenceSearch {

    pub fn new(move_gen: MoveGen, evaluator: Evaluator) -> Self {
        Self {
            depth: 0,
            evaluator,
            move_gen,
        }
    }

    pub fn search(
        &self,
        board: &mut Board,
        movegen: &MoveGen,
        mut alpha: i32,
        beta: i32,
        white_to_move: bool,
        ply: u8
    ) -> i32 {

        // what the wiki says ! (I will not be listening)
        // Step 1: implement s MVV-LVA before search to prevent search explosion
        // step 2: static exchange & delta pruning
        // step 3: qsearch

        const MAX_QSEARCH_DEPTH: u8 = 10;  // clamp to 10 so we don't destroy our computer

        if ply >= MAX_QSEARCH_DEPTH {
            return Evaluator::evaluate(board, white_to_move);
        }

        let static_eval = Evaluator::evaluate(board, white_to_move);

        if ply >= MAX_QSEARCH_DEPTH {
            return Evaluator::evaluate(board, white_to_move);
        }

        if static_eval >= beta {
            return beta;
        }
        alpha = alpha.max(static_eval);

        let static_eval = Evaluator::evaluate(&board, white_to_move);

        if static_eval >= beta {
            return beta;
        }
        alpha = alpha.max(static_eval);

        // Delta pruning -> only continue if a capture could improve alpha
        // https://talkchess.com/viewtopic.php?t=80325
        // we can model this as an equation with
        // if(statc_eval + Δ < α) return α
        if static_eval + MaterialEvaluator::QUEEN_VALUE + 200 < alpha {
            return alpha
        }

        // if we find beta cutoff earlier we stop looping
        // therefore this saves us time
        let mut capture_moves = movegen.generate_capture_moves(board, white_to_move);
        capture_moves.sort_by_cached_key(|m| -m.score_capture(board));

        for mv in capture_moves {
            // TODO
            // If ( losing capture ) {
            //     continue
            //  }

            let captured = board.piece_at(mv.to).map(|(pt, _)| pt);
            board.make_move(&mv, white_to_move);

            let score = -self.search(
                board,
                movegen,
                -beta,
                -alpha,
                !white_to_move,
                ply + 1
            );

            board.unmake_move(&mv, white_to_move, captured);

            if score >= beta {
                return beta;
            }

            alpha = alpha.max(score);
        }

        alpha
    }

}