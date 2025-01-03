use crate::eval2::eval;

use chess::{Board, BoardStatus, ChessMove, MoveGen, Piece, EMPTY};

const MATE_VALUE: f32 = 1000000000.;

pub fn alpha_beta_search2(board: &Board, max_depth: u32) -> (f32, ChessMove) {
    // This is so shitty, but it works
    let move_gen_moves = MoveGen::new_legal(board);
    let mut chess_moves = Vec::new();
    for chess_move in move_gen_moves {
        chess_moves.push((0., chess_move));
    }
    for depth in 1..max_depth {
        chess_moves = n_first_layer(&board, depth, f32::NEG_INFINITY, f32::INFINITY, &chess_moves);
        chess_moves.sort_by(|a, b| b.0.total_cmp(&a.0));
    }
    *chess_moves.iter().max_by(|a, b| a.0.total_cmp(&b.0)).unwrap()
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
    mut a: f32,
    b: f32,
    move_order: &Vec<(f32, ChessMove)>
) -> Vec<(f32, ChessMove)> {
    let mut value = f32::NEG_INFINITY;
    let mut output: Vec<(f32, ChessMove)> = Vec::new();

    let mut process_move = |chess_move: ChessMove| -> bool {
        let child_board = board.make_move_new(chess_move);
        let mut new_value = n_max(&child_board, depth - 1, -b, -a);
        new_value = -new_value;
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

fn n_max(board: &Board, depth: u32, mut a: f32, b: f32) -> f32 {
    let mut value;
    if board.status() == BoardStatus::Checkmate {
        return -MATE_VALUE - depth as f32;
    }
    if only_kings(board) || board.status() == BoardStatus::Stalemate {
        return 0.;
    }
    if depth == 0 {
        let value = eval(board);
        return value;
    }

    let mut chess_moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    chess_moves.set_iterator_mask(*targets); // This makes the capture moves be first.

    let first_move = match chess_moves.next() {
        Some(mv) => mv,
        None => {
            chess_moves.set_iterator_mask(!EMPTY);
            chess_moves.next().unwrap()
        }
    };

    let mut current_move;

    // Process first move
    {
        let child_board = board.make_move_new(first_move);
        let new_value = n_max(&child_board, depth - 1, -b, -a);
        value = -new_value;
        current_move = first_move;
        a = a.max(value);
    }

    let mut process_move = |chess_move: ChessMove| -> bool {
        let child_board = board.make_move_new(chess_move);
        let mut new_value = n_max(&child_board, depth - 1, -b, -a);
        new_value = -new_value;
        if new_value > value {
            value = new_value;
            current_move = chess_move;
            a = a.max(value);
        }
        a >= b
    };

    for chess_move in &mut chess_moves {
        if process_move(chess_move) {
            //println!("a: {}, b: {}", a, b);
            return value;
        }
    }
    chess_moves.set_iterator_mask(!EMPTY);
    for chess_move in &mut chess_moves {
        if process_move(chess_move) {
            //println!("a: {}, b: {}", a, b);
            return value;
        }
    }

    value
}
