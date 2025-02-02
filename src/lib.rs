pub mod graph;
mod utils;

extern crate modulee_engine;
pub use graph::Graph;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, modulee-engine-wasm!");
}
