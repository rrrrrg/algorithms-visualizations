use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Boundary {
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}
