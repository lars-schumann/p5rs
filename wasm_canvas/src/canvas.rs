use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, window};

pub struct Canvas {
    width: u32,
    height: u32,
    canvas_elem: HtmlCanvasElement,
    canvas_context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        match (width, height) {
            (..=2000, ..=2000) => {
                let canvas_elem = get_canvas();
                let canvas_context = get_canvas_context();
                if canvas_elem.width() != width {
                    canvas_elem.set_width(width);
                }
                if canvas_elem.height() != height {
                    canvas_elem.set_height(height);
                }
                Ok(Self {
                    width,
                    height,
                    canvas_elem,
                    canvas_context,
                })
            }
            (2001.., _) => Err(String::from("canvas_width has to be in range [0,2000]")),
            (_, 2001..) => Err(String::from("canvas_height has to be in range [0,2000]")),
        }
    }

    pub fn draw_rect(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        self.canvas_context.set_fill_style_str(color);
        self.canvas_context.fill_rect(x, y, width, height);
    }
}

fn get_document() -> Document {
    window().unwrap().document().unwrap()
}

fn get_canvas() -> HtmlCanvasElement {
    get_document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

fn get_canvas_context() -> CanvasRenderingContext2d {
    get_canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}
