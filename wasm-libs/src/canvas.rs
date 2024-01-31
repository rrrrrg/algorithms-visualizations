use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Velocity {
    pub dx: i32,
    pub dy: i32,
}

#[wasm_bindgen]
impl Velocity {
    pub fn new(dx: i32, dy: i32) -> Velocity {
        Velocity { dx, dy }
    }
}

#[wasm_bindgen]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}
