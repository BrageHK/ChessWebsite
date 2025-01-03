use wasm_bindgen::prelude::*;

// This is needed for WASM initialization
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();
}
