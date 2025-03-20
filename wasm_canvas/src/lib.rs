use std::{iter::RepeatN, thread::sleep, time};

use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement,
    js_sys::{Math::random, eval},
    window,
};

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
    context.set_fill_style(&"yellow".into());
    context.fill_rect(50.0, 50.0, 55.0, 55.0);
}

#[wasm_bindgen]
pub fn noomber() {
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

    context.set_fill_style(&"blue".into());
    let (x, y) = (500. * random(), 500. * random());
    context.fill_rect(x, y, 1., 1.);

    web_sys::console::log1(&String::from("wassup"));
}

//wasm-pack build --target web --out-dir ../static/wasm
