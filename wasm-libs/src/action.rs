use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}
#[wasm_bindgen]
impl Velocity {
    #[wasm_bindgen(constructor)]
    pub fn new(dx: f64, dy: f64) -> Velocity {
        Velocity { dx, dy }
    }
}

pub trait Move {
    fn moving(&mut self, velocity: Velocity);
}
