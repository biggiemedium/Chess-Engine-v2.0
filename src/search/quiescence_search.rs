
//Most chess programs, at the end of the main search perform a more limited quiescence search
// containing fewer moves

use crate::board::board::Board;
use crate::evaluation::evaluator::Evaluator;
use crate::evaluation::materials::MaterialEvaluator;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

// The purpose of Quiescence search is to only evaluate "quiet" positions,
// or positions where there are no winning tactical moves to be made
struct QuiescenceSearch {
    depth: i8, // to prevent search "explosion"
    evaluator: Evaluator,
    move_gen: MoveGen
}

impl QuiescenceSearch {

    pub fn search(&self, board: &mut Board, mut alpha: i32, mut beta: i32, mut depth: u32, white_to_move: bool) -> i32 {

        // what the wiki says ! (I will not be listening)
        // Step 1: implement s MVV-LVA before search to prevent search explosion
        // step 2: static exchange & delta pruning
        // step 3: qsearch

        if(depth > 10) {
            depth = 10; // clamp to 10 so we don't destroy our computer
        }

        // Stop searching once depth reaches 0
        if depth == 0 {
            return Evaluator::evaluate(&board);
        }

        let static_eval = Evaluator::evaluate(&board);

        if static_eval >= beta {
            return beta;
        }

        // Delta pruning -> only continue if a capture could improve alpha
        // https://talkchess.com/viewtopic.php?t=80325
        // we can model this as an equation with
        // if(statc_eval + Δ < α) return α
        if static_eval + MaterialEvaluator::QUEEN_VALUE + 200 < alpha {
            return alpha
        }

        // if we find beta cutoff earlier we stop looping
        // therefore this saves us time
        let mut captureMoves = self.move_gen.generate_capture_moves(&board, white_to_move);
        captureMoves.sort_by_cached_key(|m| -m.score_capture(&board));

        for mv in captureMoves {

            // TODO
            // If ( losing capture ) {
            //     continue
            //  }

            let captured = board.piece_at(mv.to).map(|(pt, _)| pt);
            board.make_move(&mv, white_to_move);

            let score = -self.search(
                board,
                -alpha,
                -beta,
                depth - 1,
                !white_to_move
            );

            board.unmake_move(&mv, white_to_move, captured);

            if(score >= beta) {
                return beta;
            }

            if(score > alpha) {
                alpha = score
            }
        }

        alpha
    }

}