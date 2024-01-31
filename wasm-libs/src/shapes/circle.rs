use crate::canvas::{Coordinate, Velocity};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Circle {
    radius: f64,
}

#[wasm_bindgen]
impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

#[wasm_bindgen]
pub struct MovingCircle {
    circle: Circle,
    coordinate: Coordinate,
    velocity: Velocity,
}

#[wasm_bindgen]
impl MovingCircle {
    pub fn new(coordinate: Coordinate, circle: Circle, velocity: Velocity) -> MovingCircle {
        MovingCircle {
            coordinate,
            circle,
            velocity,
        }
    }

    pub fn moving(&mut self) {
        self.coordinate.x += self.velocity.dx;
        self.coordinate.y += self.velocity.dy;
    }

    pub fn get_radius(&self) -> f64 {
        self.circle.radius
    }

    pub fn get_coordinate(&self) -> Coordinate {
        Coordinate {
            x: self.coordinate.x,
            y: self.coordinate.y,
        }
    }
}
