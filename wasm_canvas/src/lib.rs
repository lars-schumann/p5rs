mod types;
use types::{Float, Int};
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, js_sys::Math::random, window,
};

static P_1_X: f64 = 400.0;
static P_1_Y: f64 = 0.0;
static P_2_X: f64 = 0.0;
static P_2_Y: f64 = 800.0;
static P_3_X: f64 = 800.0;
static P_3_Y: f64 = 800.0;

static P_X: Float = Float::new(0.0);
static P_Y: Float = Float::new(0.0);

fn get_canvas_context() -> CanvasRenderingContext2d {
    let document: Document = window().unwrap().document().unwrap();

    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}
#[wasm_bindgen(start)]
pub fn start() {}

#[wasm_bindgen]
pub fn setup() {
    draw_rect(400.0, 0.0, 10.0, 10.0, "green");
    draw_rect(0.0, 800.0, 10.0, 10.0, "green");
    draw_rect(800.0, 800.0, 10.0, 10.0, "green");
}

#[wasm_bindgen]
pub fn draw() {
    for _ in 0..100 {
        match (random() * 3.0).floor() as u8 {
            0 => {
                P_X.set((P_X.get() + P_1_X) / 2.);
                P_Y.set((P_Y.get() + P_1_Y) / 2.);
            }
            1 => {
                P_X.set((P_X.get() + P_2_X) / 2.);
                P_Y.set((P_Y.get() + P_2_Y) / 2.);
            }
            _ => {
                P_X.set((P_X.get() + P_3_X) / 2.);
                P_Y.set((P_Y.get() + P_3_Y) / 2.);
            }
        }

        draw_rect(P_X.get(), P_Y.get(), 0.2, 0.2, "blue");
    }
}

fn draw_rect(x: f64, y: f64, width: f64, height: f64, color: &str) {
    let context = get_canvas_context();
    context.set_fill_style_str(color);
    context.fill_rect(x, y, width, height);
}
//wasm-pack build --target web --out-dir ../static/wasm
