use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]

pub fn make_chess_move(fen: &str) -> String {
    format!("Received FEN: {}. Suggesting move: e2e4", fen)
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
