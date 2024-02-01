use web_sys::CanvasRenderingContext2d;

pub trait Draw {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
}

pub mod coordinate {
    use wasm_bindgen::prelude::*;

    pub type X = f64;
    pub type Y = f64;

    #[wasm_bindgen]
    pub struct Coordinate {
        pub x: f64,
        pub y: f64,
    }

    pub trait GetCoordinate {
        fn get_coordinate(&self) -> (X, Y);
    }
}
