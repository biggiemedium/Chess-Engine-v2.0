//Most chess programs, at the end of the main search perform a more limited quiescence search
// containing fewer moves

// BASED ON:
// https://github.com/biggiemedium/ChessAI/blob/master/src/main/java/dev/chess/ai/Engine/Quiescence/QuiescenceSearch.java

use crate::board::board::{Board, PieceType};
use crate::evaluation::evaluator::Evaluator;
use crate::evaluation::materials::MaterialEvaluator;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

// The purpose of Quiescence search is to only evaluate "quiet" positions,
// or positions where there are no winning tactical moves to be made
pub struct QuiescenceSearch {
    depth: i8, // to prevent search "explosion"
    evaluator: Evaluator,
    move_gen: MoveGen,
}

impl QuiescenceSearch {

    const BIG_DELTA: i32 = MaterialEvaluator::QUEEN_VALUE + 200;

    pub fn new(move_gen: MoveGen, evaluator: Evaluator) -> Self {
        Self {
            depth: 0,
            evaluator,
            move_gen,
        }
    }

    /// Worst-case: O(N^D) where N is number of captures per position and D is MAX_QSEARCH_DEPTH.
    /// With alpha-beta pruning, average case approaches O(N^(D/2))
    pub fn search(
        board: &mut Board,
        movegen: &MoveGen,
        mut alpha: i32,
        beta: i32,
        white_to_move: bool,
        ply: u8,
    ) -> i32 {

        // what the wiki says ! (I will not be listening)
        // Step 1: implement s MVV-LVA before search to prevent search explosion
        // step 2: static exchange & delta pruning
        // step 3: qsearch

        const MAX_QSEARCH_DEPTH: u8 = 10;  // clamp to 10 so we don't destroy our computer

        if ply >= MAX_QSEARCH_DEPTH {
            return Evaluator::evaluate(board, white_to_move);
        }

        let in_check = movegen.is_king_in_check(board, white_to_move);

        // Stand pat: use static eval as a lower bound on the score.
        // Not allowed when in check — position is not quiet, we must resolve the threat.
        let stand_pat = Evaluator::evaluate(board, white_to_move);
        if !in_check {
            if stand_pat >= beta {
                return beta;
            }

            // Delta pruning -> only continue if a capture could improve alpha
            // https://talkchess.com/viewtopic.php?t=80325
            // we can model this as an equation with
            // if(static_eval + Δ < α) return α
            if stand_pat + Self::BIG_DELTA < alpha {
                return alpha;
            }

            alpha = alpha.max(stand_pat);
        }

        // If in check: generate all evasions so we don't miss a forced mate
        // Otherwise:   captures only
        let mut capture_moves: Vec<Move> = if in_check {
            let mut evasions = Vec::new();
            movegen.generate_moves(board, white_to_move, &mut evasions);
            evasions
        } else {
            movegen.generate_capture_moves(board, white_to_move)
        };

        // No moves while in check -> checkmate
        // No captures in a quiet position -> return stand-pat
        if capture_moves.is_empty() {
            return if in_check {
                -(1_000_000 - ply as i32)
            } else {
                stand_pat
            };
        }

        // if we find beta cutoff earlier we stop looping
        // therefore this saves us time
        capture_moves.sort_by_cached_key(|m| -m.score_capture(board));

        // best_score starts at stand_pat so we never return something worse than doing nothing.
        // When in check we have no standing option, so start at -INF.
        let mut best_score = if in_check { -1_000_000 } else { stand_pat };

        for mv in capture_moves {

            // TODO
            // If ( losing capture ) {
            //     continue
            // }

            // Skip obviously losing captures via SEE (e.g. QxP defended by pawn)
            // Only prune when not in check — in check we must consider all evasions
            if !in_check && board.piece_at(mv.to).is_some() {
                if Self::static_exchange_evaluation(board, movegen, &mv, white_to_move) < 0 {
                    continue;
                }
            }

            let captured = board.piece_at(mv.to).map(|(pt, _)| pt);
            board.make_move(&mv, white_to_move);

            let score = -Self::search(
                board,
                movegen,
                -beta,
                -alpha,
                !white_to_move,
                ply + 1,
            );

            board.unmake_move(&mv, white_to_move, captured);

            if score > best_score {
                best_score = score;
            }

            if score >= beta {
                return beta; // fail-hard cutoff
            }

            alpha = alpha.max(score);
        }

        best_score
    }

    /// Simulates a series of captures on a square and returns the material
    /// gain/loss for the side initiating the capture.
    /// Negative -> losing capture (skip it), positive/zero -> acceptable.
    ///
    /// https://www.chessprogramming.org/Static_Exchange_Evaluation
    /// O(32 * C) where C is the number of capture moves per position, effectively O(C).
    pub fn static_exchange_evaluation(
        board: &mut Board,
        movegen: &MoveGen,
        capture: &Move,
        by_white: bool,
    ) -> i32 {
        let target_sq = capture.to;

        let captured_value = match board.piece_at(target_sq) {
            Some((pt, _)) => MaterialEvaluator::piece_type_value(pt),
            None => return 0,
        };

        let attacker_value = match board.piece_at(capture.from) {
            Some((pt, _)) => MaterialEvaluator::piece_type_value(pt),
            None => return 0,
        };

        // gain[0] = what we immediately win by making the first capture
        let mut gain = [0i32; 32];
        gain[0] = captured_value;

        let mut depth = 0usize;
        let mut current_attacker_value = attacker_value;
        let mut current_turn = !by_white; // defender recaptures next

        let original_captured = board.piece_at(capture.to).map(|(pt, _)| pt);
        board.make_move(capture, by_white);

        // Track every move made so we can unwind them all afterwards
        let mut move_stack: Vec<(Move, bool, Option<PieceType>)> =
            vec![(capture.clone(), by_white, original_captured)];

        loop {
            depth += 1;
            if depth >= 31 {
                break;
            }

            // Find the cheapest attacker for the current side
            let Some((recapture_mv, recapturer_value)) =
                Self::find_cheapest_attacker(board, movegen, target_sq, current_turn)
            else {
                break; // No more attackers
            };

            gain[depth] = current_attacker_value - gain[depth - 1];
            current_attacker_value = recapturer_value;

            let cap_on_sq = board.piece_at(target_sq).map(|(pt, _)| pt);
            board.make_move(&recapture_mv, current_turn);
            move_stack.push((recapture_mv, current_turn, cap_on_sq));

            current_turn = !current_turn;
        }

        // Unwind all simulated moves in reverse order
        for (mv, side, captured_pt) in move_stack.into_iter().rev() {
            board.unmake_move(&mv, side, captured_pt);
        }

        // Minimax the gain array back to gain[0]
        // "Did I actually profit after my opponent plays optimally?"
        let mut i = depth as isize;
        while i > 0 {
            gain[(i - 1) as usize] = -(-gain[(i - 1) as usize]).max(gain[i as usize]);
            i -= 1;
        }

        gain[0]
    }

    /// Returns the cheapest attacker for `side` targeting `target_sq`, and its material value.
    /// O(C) where C is the number of capture moves generated for the given side.
    fn find_cheapest_attacker(
        board: &Board,
        movegen: &MoveGen,
        target_sq: u8,
        side: bool,
    ) -> Option<(Move, i32)> {
        movegen
            .generate_capture_moves(board, side)
            .into_iter()
            .filter(|mv| mv.to == target_sq)
            .filter_map(|mv| {
                let value = board
                    .piece_at(mv.from)
                    .map(|(pt, _)| MaterialEvaluator::piece_type_value(pt))?;
                Some((mv, value))
            })
            .min_by_key(|(_, value)| *value)
    }
}