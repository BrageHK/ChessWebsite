use chess::Board;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

mod alpha_beta;
mod eval;

#[derive(Serialize)]
struct MoveResult {
    chess_move: String,
    score: i32,
}

#[wasm_bindgen]
pub fn make_chess_move(fen: &str, depth: u32) -> JsValue {
    let board = Board::from_str(fen).unwrap();
    let (score, chess_move) = alpha_beta::alpha_beta_search(&board, depth);

    let result = MoveResult {
        chess_move: chess_move.to_string(),
        score,
    };

    to_value(&result).unwrap()
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
