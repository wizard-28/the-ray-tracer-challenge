#![warn(clippy::pedantic)]

use the_ray_tracer_challenge::{
    canvas::Canvas,
    primitive::{Color, Point, Tuple, Vector},
};

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut canvas = Canvas::new(8 * 10, 50);
    let mut proj = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 3.0,
    };
    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    loop {
        proj = tick(&env, &proj);
        if proj.position.y() <= 0.0 {
            break;
        }
        let x = proj.position.x().abs();
        let y = (proj.position.y() - f64::from(canvas.height())).abs();

        canvas.write_pixel(x as u32, y as u32, Color::red());
    }

    canvas.export("image.png").unwrap();
}
