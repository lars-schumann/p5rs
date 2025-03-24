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

    for _ in 0..1_000 {
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
        canvas.draw_circ(mouse.location, 10., "red");
    } else {
        canvas.draw_circ(mouse.location, 1., "red");
    }

    log(&mouse);

    POINTS.set(points);
    CURRENT.set(current);
}
