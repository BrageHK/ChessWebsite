use crate::eval::eval;

use chess::{Board, BoardStatus, ChessMove, MoveGen, Piece, EMPTY};

use web_sys;

const MATE_VALUE: i32 = 1000000000;

pub fn alpha_beta_search(board: &Board, max_depth: u32) -> (i32, ChessMove) {
    let move_gen_moves = MoveGen::new_legal(board);
    let mut chess_moves = Vec::new();
    for chess_move in move_gen_moves {
        chess_moves.push((0, chess_move));
    }
    for depth in 1..max_depth {
        chess_moves = n_first_layer(board, depth, i32::MIN, i32::MAX, &chess_moves);
        chess_moves.sort_by(|a, b| b.0.cmp(&a.0));
    }
    web_sys::console::log_1(&format!("Best move!: {:?}", chess_moves).into());
    *chess_moves.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap()
}

/// Check if the game is stalemate by only having 2 kings on the board.
pub fn only_kings(board: &Board) -> bool {
    let kings = board.pieces(Piece::King);
    let all_pieces = board.combined();
    all_pieces == kings
}

// TODO: Evaluate "killer moves" i.e. capture moves first
fn n_first_layer(
    board: &Board,
    depth: u32,
    mut a: i32,
    b: i32,
    move_order: &Vec<(i32, ChessMove)>,
) -> Vec<(i32, ChessMove)> {
    let mut value = i32::MIN;
    let mut output: Vec<(i32, ChessMove)> = Vec::new();

    let mut process_move = |chess_move: ChessMove| -> bool {
        let child_board = board.make_move_new(chess_move);
        let new_value = -n_max(&child_board, depth - 1, -b, -a);
        if new_value > value {
            value = new_value;
            a = a.max(value);
        }
        output.push((new_value, chess_move));
        a >= b
    };

    for chess_move in move_order.iter() {
        process_move(chess_move.1);
    }

    output
}

fn quiescence_search(board: &Board, mut a: i32, b: i32) -> i32 {
    // Should do a standing pat evaluation first
    let stand_pat = eval(board);
    if stand_pat >= b {
        return stand_pat;
    }
    if stand_pat > a {
        a = stand_pat;
    }

    let mut value = i32::MIN;
    if board.status() == BoardStatus::Checkmate {
        return -MATE_VALUE;
    }
    if only_kings(board) || board.status() == BoardStatus::Stalemate {
        return 0;
    }
    let mut chess_moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    chess_moves.set_iterator_mask(*targets);
    if chess_moves.len() == 0 {
        return eval(board);
    } else {
        for chess_move in chess_moves {
            let child_board = board.make_move_new(chess_move);
            value = -quiescence_search(&child_board, -b, -a);
            a = a.max(value);
            if a >= b {
                break;
            }
        }
        return value;
    }
}

fn n_max(board: &Board, depth: u32, mut a: i32, b: i32) -> i32 {
    if board.status() == BoardStatus::Checkmate {
        return -MATE_VALUE - depth as i32;
    }
    if only_kings(board) || board.status() == BoardStatus::Stalemate {
        return 0;
    }
    if depth == 0 {
        return quiescence_search(board, a, b);
    }

    let mut value = i32::MIN;
    let mut chess_moves = MoveGen::new_legal(board);

    // Try captures first
    let targets = board.color_combined(!board.side_to_move());
    chess_moves.set_iterator_mask(*targets);

    for chess_move in &mut chess_moves {
        let child_board = board.make_move_new(chess_move);
        let score = -n_max(&child_board, depth - 1, -b, -a);
        if score > value {
            value = score;
            if value > a {
                a = value;
                if a >= b {
                    return value;
                }
            }
        }
    }

    // Then try non-captures
    chess_moves.set_iterator_mask(!EMPTY);
    for chess_move in &mut chess_moves {
        let child_board = board.make_move_new(chess_move);
        let score = -n_max(&child_board, depth - 1, -b, -a);
        if score > value {
            value = score;
            if value > a {
                a = value;
                if a >= b {
                    return value;
                }
            }
        }
    }

    value
}
