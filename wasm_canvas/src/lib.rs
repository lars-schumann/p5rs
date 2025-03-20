use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, window};

#[wasm_bindgen(start)]
pub fn start() {
    let document: Document = window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Draw a rectangle
    context.set_fill_style(&"blue".into());
    context.fill_rect(50.0, 50.0, 100.0, 100.0);
}
