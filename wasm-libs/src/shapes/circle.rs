use crate::{
    action::{Move, Velocity},
    canvas::Draw,
};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Circle {
    color: String,
    radius: f64,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Circle {
    pub fn new(color: String, radius: f64, x: f64, y: f64) -> Circle {
        Circle {
            color,
            radius,
            x,
            y,
        }
    }
}

impl Move for Circle {
    fn moving(&mut self, velocity: Velocity) {
        self.x += velocity.dx;
        self.y += velocity.dy;
    }
}

impl Draw for Circle {
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();

        ctx.arc(self.x, self.y, self.radius, 0.0, std::f64::consts::PI * 2.0)
            .expect("arc failed while drawing a circle.");

        ctx.set_fill_style(&JsValue::from_str(self.color.as_str()));
        ctx.fill();
    }
}
