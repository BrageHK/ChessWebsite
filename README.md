# Project Name

A WebAssembly project built with Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Building

1. Install wasm-pack if you haven't already: 

```bash
cargo install wasm-pack
```
2. Build the WebAssembly package:

```bash
wasm-pack build --target web
```

3. Run with python 
```bash
python3 -m http.server
```

4. Open the browser and go to http://localhost:8000/
