use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlDivElement, window};

use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub struct Mouse {
    pub location: Point,
    pub is_pressed: bool,
}

impl Mouse {
    pub fn get() -> Self {
        Self {
            location: Point {
                x: get_mouse_x(),
                y: get_mouse_y(),
            },
            is_pressed: get_mouse_is_pressed(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = mouseX)]
    fn get_mouse_x() -> f64;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = mouseY)]
    fn get_mouse_y() -> f64;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = mouseIsPressed)]
    fn get_mouse_is_pressed() -> bool;
}
