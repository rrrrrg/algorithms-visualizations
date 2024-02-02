use crate::{
    action::Velocity,
    canvas::{Boundary, Coordinate},
};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Circle {
    color: String,
    radius: f64,
    coordinate: Coordinate,
    boundary: Boundary,
    velocity: Velocity,
}

#[wasm_bindgen]
impl Circle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: String,
        radius: f64,
        coordinate: Coordinate,
        boundary: Boundary,
        velocity: Velocity,
    ) -> Circle {
        Circle {
            color,
            radius,
            coordinate,
            boundary,
            velocity,
        }
    }

    #[wasm_bindgen]
    pub fn moving(&mut self) {
        self.coordinate.x += self.velocity.dx;
        self.coordinate.y += self.velocity.dy;
    }

    #[wasm_bindgen]
    pub fn set_random_velocity(&mut self) {}

    #[wasm_bindgen]
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();

        ctx.arc(
            self.coordinate.x,
            self.coordinate.y,
            self.radius,
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .expect("arc failed while drawing a circle.");

        ctx.set_fill_style(&JsValue::from_str(self.color.as_str()));
        ctx.fill();
    }

    #[wasm_bindgen]
    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate.clone()
    }
}
