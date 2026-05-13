use crate::board::board::Board;
use crate::evaluation::evaluator::Evaluator;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;

pub struct Search;

impl Search {
    const NEG_INFINITY: i32 = -1_000_000;
    const POS_INFINITY: i32 = 1_000_000;

    // https://github.com/biggiemedium/ChessAI/blob/master/src/main/java/dev/chess/ai/Engine/Search/impl/AlphaBetaAlgorithm.java
    pub fn find_best_move(
        movegen: &MoveGen,
        board: &Board,
        white_to_move: bool,
        depth: u8,
    ) -> Option<Move> {

        // generate all possible moves
        // TODO: filter shit moves out so we don't waste computation time
        let mut moves = Vec::new();
        movegen.generate_moves(board, white_to_move, &mut moves);

        if moves.is_empty() {
            return None;
        }

        let mut best_move = moves[0];
        let mut best_score = Self::NEG_INFINITY;

        for mv in moves {
            let mut board_copy = board.clone();
            let captured = board_copy.piece_at(mv.to).map(|(pt, _)| pt);

            board_copy.make_move(&mv, white_to_move);
            let score = -Self::AlphaBeta(
                movegen,
                &mut board_copy,
                !white_to_move,
                depth - 1,
                Self::NEG_INFINITY,
                Self::POS_INFINITY,
            );

            board_copy.unmake_move(&mv, white_to_move, captured);

            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }

        Some(best_move)
    }

    pub fn AlphaBeta(
        movegen: &MoveGen,
        board: &mut Board,
        white_to_move: bool,
        depth: u8,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {

        if depth == 0 {
            let eval = Evaluator::evaluate(board);
            return if white_to_move {
                eval
            } else {
                -eval
            };
        }

        let mut moves = Vec::new();
        movegen.generate_moves(board, white_to_move, &mut moves);

        if moves.is_empty() {
            // Checkmate / stalemate
            // TODO: Replace this
            return Self::NEG_INFINITY + 1;
        }

        let mut max_score = Self::NEG_INFINITY;

        for mv in moves {
            let captured = board.piece_at(mv.to).map(|(pt, _)| pt);
            board.make_move(&mv, white_to_move);

            let score = -Self::AlphaBeta(
                movegen,
                board,
                !white_to_move,
                depth - 1,
                -beta,
                -alpha,
            );

            board.unmake_move(&mv, white_to_move, captured);

            max_score = max_score.max(score);
            alpha = alpha.max(score);

            if alpha >= beta {
                break;
            }
        }
        max_score
    }
}