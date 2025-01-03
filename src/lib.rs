use std::str::FromStr;

use wasm_bindgen::prelude::*;
use chess::Board;

mod alpha_beta;
mod eval;

#[wasm_bindgen]
pub fn make_chess_move(fen: &str, depth: u32) -> String {
    let board = Board::from_str(fen).unwrap();
    let (score, chess_move) = alpha_beta::alpha_beta_search(&board, depth);

    format!("Received FEN: {}. Suggesting move: {}. Score: {}", fen, chess_move.to_string(), score)
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
