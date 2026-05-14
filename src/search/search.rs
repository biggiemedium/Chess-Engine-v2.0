use std::cmp;
use crate::board::board::Board;
use crate::evaluation::evaluator::Evaluator;
use crate::movegen::movegeneration::MoveGen;
use crate::movegen::r#move::Move;
use crate::table::transpositiontable::TranspositionTable;
use crate::search::quiescence_search::QuiescenceSearch;

pub struct Search {
    transposition: TranspositionTable,
    nodesSearched: u64,
    quiescence_search: QuiescenceSearch
}

impl Search {

    pub fn new() -> Self {
        Self {
            transposition: TranspositionTable::new(1_000_000),
            nodesSearched: 0,
            quiescence_search: QuiescenceSearch::new(MoveGen::new(), Evaluator),
        }
    }

    const NEG_INFINITY: i32 = -1_000_000;
    const POS_INFINITY: i32 = 1_000_000;

    // https://github.com/biggiemedium/ChessAI/blob/master/src/main/java/dev/chess/ai/Engine/Search/impl/AlphaBetaAlgorithm.java
    pub fn find_best_move(
        &mut self,
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

        moves.sort_by_cached_key(|m| {
            if board.piece_at(m.to).is_some() {
                0  // Captures first
            } else {
                1  // Quiet moves second
            }
        });

        let mut best_move = moves[0];
        let mut best_score = Self::NEG_INFINITY;
        let mut alpha = Self::NEG_INFINITY;
        let beta = Self::POS_INFINITY;

        for mv in moves {
            let mut board_copy = board.clone();
            let captured = board_copy.piece_at(mv.to).map(|(pt, _)| pt);

            board_copy.make_move(&mv, white_to_move);
            let score = -self.AlphaBeta(
                movegen,
                &mut board_copy,
                !white_to_move,
                depth - 1,
                -beta,
                -alpha,
            );

            board_copy.unmake_move(&mv, white_to_move, captured);

            if score > best_score {
                best_score = score;
                best_move = mv;
            }

            alpha = cmp::max(alpha, score);
        }

        Some(best_move)
    }

    pub fn AlphaBeta(
        &mut self,
        movegen: &MoveGen,
        board: &mut Board,
        white_to_move: bool,
        depth: u8,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        self.nodesSearched += 1;

        let zobrist_hash = board.zobrist_hash();

        // Probe transposition table
        if let Some(entry) = self.transposition.probe(zobrist_hash) {

            if entry.depth >= depth {
                match entry.flag {
                    TranspositionTable::EXACT => {
                        return entry.score;
                    }

                    TranspositionTable::LOWER => {
                        if entry.score >= beta {
                            return entry.score;
                        }
                        alpha = alpha.max(entry.score);
                    }

                    TranspositionTable::UPPER => {
                        if entry.score <= alpha {
                            return entry.score;
                        }
                    }

                    _ => {}
                }

                if alpha >= beta {
                    return entry.score;
                }
            }
        }

        if depth == 0 {
            return self.quiescence_search.search(board, movegen, alpha, beta, white_to_move, 0);
        }

        let original_alpha = alpha;
        let mut moves = Vec::new();
        movegen.generate_moves(board, white_to_move, &mut moves);

        if moves.is_empty() {
            // If king is in check and no legal moves -> checkmate
            // If king is not in check and no legal moves -> stalemate
            return if movegen.is_king_in_check(board, white_to_move) {
                -(Self::POS_INFINITY - depth as i32)  // Checkmate -> (prefer faster mates)
            } else {
                0  // Stalemate (draw)
            };
        }

        moves.sort_by_cached_key(|m| {
            if board.piece_at(m.to).is_some() {
                0  // Captures first
            } else {
                1  // Quiet moves second
            }
        });

        let mut best_move = moves[0];
        let mut max_score = Self::NEG_INFINITY;

        for mv in moves {
            let captured = board.piece_at(mv.to).map(|(pt, _)| pt);
            board.make_move(&mv, white_to_move);

            let score = -self.AlphaBeta(
                movegen,
                board,
                !white_to_move,
                depth - 1,
                -beta,
                -alpha,
            );

            board.unmake_move(&mv, white_to_move, captured);

            if score > max_score {
                max_score = score;
                best_move = mv;
            }

           // max_score = max_score.max(score);
            alpha = alpha.max(score);

            if alpha >= beta {
                break;
            }
        }

        // Determine node type
        let flag = if max_score <= original_alpha {
            TranspositionTable::UPPER
        } else if max_score >= beta {
            TranspositionTable::LOWER
        } else {
            TranspositionTable::EXACT
        };

        self.transposition.store(
            zobrist_hash,
            max_score,
            depth,
            flag as u8,
            best_move,
            0,
        );
        max_score
    }
}