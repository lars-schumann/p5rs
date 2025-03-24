mod draw;
mod edit;
mod global;
mod mouse;
mod point;
mod util;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn setup() {
    edit::setup();
}

#[wasm_bindgen]
pub fn draw() {
    edit::draw()
}

//wasm-pack build --target web --out-dir ./wasm
