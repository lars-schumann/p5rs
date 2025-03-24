use std::f64::consts::PI;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, window};

use crate::point::*;

#[derive(Debug)]
pub struct Canvas {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn get() -> Self {
        Canvas {
            element: get_canvas_element(),
            context: get_canvas_context(),
        }
    }
}

pub trait Drawable {
    fn draw_rect(&self, point: Point, width: f64, height: f64, color: &str);
    fn draw_circ(&self, point: Point, radius: f64, color: &str);
    fn clear(&self);
    fn translate(&self, dx: f64, dy: f64);
}

impl Drawable for Canvas {
    fn draw_rect(&self, point: Point, width: f64, height: f64, color: &str) {
        self.context.set_fill_style_str(color);
        self.context.fill_rect(point.x, point.y, width, height);
    }

    fn draw_circ(&self, point: Point, radius: f64, color: &str) {
        self.context.set_fill_style_str(color);
        self.context.begin_path();
        self.context
            .arc(point.x, point.y, radius, 0., 2. * PI)
            .unwrap();
        self.context.stroke();
    }

    fn clear(&self) {
        self.element.set_width(self.element.width());
    }

    fn translate(&self, dx: f64, dy: f64) {
        self.context.translate(dx, dy).unwrap();
    }
}

fn get_document() -> Document {
    window().unwrap().document().unwrap()
}

fn get_canvas_element() -> HtmlCanvasElement {
    get_document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

fn get_canvas_context() -> CanvasRenderingContext2d {
    get_canvas_element()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}
