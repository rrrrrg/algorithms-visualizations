use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Boundary {
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
impl Boundary {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Boundary {
        Boundary { width, height }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Coordinate {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Coordinate {
        Coordinate { x, y }
    }
}
