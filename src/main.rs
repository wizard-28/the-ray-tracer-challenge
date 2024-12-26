#![warn(clippy::pedantic)]

use std::{thread, time::Duration};

use the_ray_tracer_challenge::primitive::{Point, Tuple, Vector};

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

    thread::sleep(Duration::from_secs(1));
    Projectile { position, velocity }
}

fn main() {
    let mut proj = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };
    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    loop {
        dbg!(&proj);
        proj = tick(&env, &proj);
        if proj.position.y() <= 0.0 {
            break;
        }
    }
}
