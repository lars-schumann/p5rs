mod edit {
    use std::collections::VecDeque;

    use crate::draw::*;
    use crate::global::*;
    use crate::mouse::*;
    use crate::point::*;
    use crate::util::*;

    static POINTS: Global<[Point; 3]> = Global::new([
        Point::new(500., 0.),
        Point::new(0., 1000.),
        Point::new(1000., 1000.),
    ]);

    static CURRENT: Global<Point> = Global::new(Point::origin());
    static RINGS: Global<VecDeque<(Point, f64)>> = Global::new(VecDeque::new());
    static MOUSE_DURATION: Global<u64> = Global::new(0);

    pub fn setup() {
        let canvas = Canvas::get();
        let points = POINTS.get();

        //canvas.translate(480., 500.);
        canvas.draw_rect(points[0], 10.0, 10.0, "red");
        canvas.draw_rect(points[1], 10.0, 10.0, "red");
        canvas.draw_rect(points[2], 10.0, 10.0, "red");
    }

    pub fn draw() {
        let canvas = Canvas::get();
        let mouse = Mouse::get();

        let points = POINTS.get();
        let mut current = CURRENT.get();
        let mut rings = RINGS.get();
        let mut mouse_duration = MOUSE_DURATION.get();

        for _ in 0..10_000 {
            match random_int(0, 2) {
                0 => {
                    current = Point::lerp(current, points[0], 0.5);
                }
                1 => {
                    current = Point::lerp(current, points[1], 0.5);
                }
                _ => {
                    current = Point::lerp(current, points[2], 0.5);
                }
            }
            canvas.draw_rect(current, 0.1, 0.1, "orange")
        }

        if mouse.is_pressed {
            mouse_duration += 1;
            canvas.draw_ring(mouse.location, mouse_duration as f64, 1.0, "red");
            rings.push_front((mouse.location, mouse_duration as f64));
        } else {
            mouse_duration = 0;
        }

        if rings.len() > 100 {
            let oldest = rings.pop_back().unwrap();
            canvas.draw_ring(oldest.0, oldest.1, 3.0, "white");
        }
        //log(&mouse);
        //log(&rings.len());

        POINTS.set(points);
        CURRENT.set(current);
        RINGS.set(rings);
        MOUSE_DURATION.set(mouse_duration);
    }
}









//#######################################
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
        fn draw_ring(&self, point: Point, radius: f64, width: f64, color: &str);
        fn clear(&self);
        fn translate(&self, dx: f64, dy: f64);
    }

    impl Drawable for Canvas {
        fn draw_rect(&self, point: Point, width: f64, height: f64, color: &str) {
            self.context.set_fill_style_str(color);
            self.context.fill_rect(point.x, point.y, width, height);
        }

        fn draw_ring(&self, point: Point, radius: f64, width: f64, color: &str) {
            self.context.set_stroke_style_str(color); // Set outline color
            self.context.set_line_width(width);
            self.context.begin_path();
            self.context
                .arc(point.x, point.y, radius, 0., 2. * PI)
                .unwrap();
            self.context.stroke();
            self.context.set_line_width(1.0);
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
}
mod global {
    use std::sync::Mutex;

    pub struct Global<T> {
        inner: Mutex<T>,
    }

    impl<T: Clone> Global<T> {
        pub const fn new(value: T) -> Self {
            Self {
                inner: Mutex::new(value),
            }
        }

        pub fn get(&self) -> T {
            (*self.inner.lock().unwrap()).clone()
        }

        pub fn set(&self, value: T) {
            *self.inner.lock().unwrap() = value;
        }
    }
}
mod mouse {
    use crate::point::*;
    use wasm_bindgen::prelude::*;

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
}
mod point {
    use crate::util::*;

    #[derive(Clone, Copy, Debug)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub const fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }

        pub const fn origin() -> Self {
            Self::new(0.0, 0.0)
        }

        pub fn lerp(start: Point, end: Point, t: f64) -> Self {
            match t {
                0.0..=1.0 => start * (1. - t) + end * t,
                _ => {
                    log(&"lerp: t was out of range 0..=1".to_string());
                    log(&t);
                    panic!();
                }
            }
        }
    }

    impl std::ops::Add for Point {
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Self::new(self.x + other.x, self.y + other.y)
        }
    }

    impl std::ops::Sub for Point {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self::new(self.x - other.x, self.y - other.y)
        }
    }

    impl std::ops::Mul<f64> for Point {
        type Output = Self;
        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(self.x * rhs, self.y * rhs)
        }
    }
}
mod util {
    pub fn log<T: std::fmt::Debug>(thing: &T) {
        web_sys::console::log_1(&format!("logging: {:?}", thing).into());
    }

    pub fn random_int(lower_bound: i64, upper_bound: i64) -> i64 {
        let range = upper_bound - lower_bound + 1;
        (web_sys::js_sys::Math::random() * range as f64).floor() as i64 + lower_bound
    }
}
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn setup() {
    edit::setup();
}
#[wasm_bindgen]
pub fn draw() {
    edit::draw()
}
//wasm-pack build --target web
