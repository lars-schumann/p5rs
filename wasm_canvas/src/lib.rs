mod types;
use types::*;

mod canvas;
use canvas::*;

mod util;
use util::*;

use wasm_bindgen::prelude::*;

static P_1_X: f64 = 500.0;
static P_1_Y: f64 = 0.0;
static P_2_X: f64 = 0.0;
static P_2_Y: f64 = 1000.0;
static P_3_X: f64 = 1000.0;
static P_3_Y: f64 = 1000.0;

static P_X: Float = Float::new(0.0);
static P_Y: Float = Float::new(0.0);

//#[wasm_bindgen(start)]
//pub fn start() {}

#[wasm_bindgen]
pub fn setup() {
    let canvas = Canvas::new(1000, 1000).unwrap();
    canvas.draw_rect(500.0, 0.0, 10.0, 10.0, "green");
    canvas.draw_rect(0.0, 995.0, 10.0, 10.0, "green");
    canvas.draw_rect(995.0, 995.0, 10.0, 10.0, "green");
}

#[wasm_bindgen]
pub fn draw() {
    let canvas = Canvas::new(1000, 1000).unwrap();
    for _ in 0..1000 {
        match random_int(0, 2) {
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

        canvas.draw_rect(P_X.get(), P_Y.get(), 0.1, 0.1, "red");
    }
    log(&P_X);
}

//wasm-pack build --target web --out-dir ../static/wasm
