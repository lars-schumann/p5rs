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
