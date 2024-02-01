use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

pub trait Move {
    fn moving(&mut self, velocity: Velocity);
}
