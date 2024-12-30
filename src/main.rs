#![warn(clippy::pedantic)]

use std::f64::consts::PI;

use the_ray_tracer_challenge::{
    canvas::Canvas,
    primitive::{Color, Point, Transform, Tuple},
};

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn main() {
    let mut canvas = Canvas::new(100, 100);
    let mut point = Point::new(25.0, 25.0, 0.0);

    for _ in 1..=12 {
        canvas.write_pixel(
            (point.x() + 50.0) as u32,
            (point.y() + 50.0) as u32,
            Color::white(),
        );
        point = point.rotate_z(2.0 * PI / 12.0);
    }

    canvas.export("image.png").unwrap();
}
