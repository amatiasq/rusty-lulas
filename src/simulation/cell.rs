use rand::{rngs::ThreadRng, Rng};

use super::vector::Vector;

pub struct Cell {
    pub position: Vector,
    pub velocity: Vector,
}

impl Cell {
    pub fn new(rng: &mut ThreadRng, width: f64, height: f64) -> Cell {
        Cell {
            position: Vector {
                x: rng.gen_range(0.0..width),
                y: rng.gen_range(0.0..height),
            },
            velocity: Vector {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
            },
        }
    }
}
