use crate::{action::Velocity, canvas::coordinate::Coordinate};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Circle {
    color: String,
    radius: f64,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Circle {
    #[wasm_bindgen(constructor)]
    pub fn new(color: String, radius: f64, x: f64, y: f64) -> Circle {
        Circle {
            color,
            radius,
            x,
            y,
        }
    }

    #[wasm_bindgen]
    pub fn moving(&mut self, velocity: Velocity) {
        self.x += velocity.dx;
        self.y += velocity.dy;
    }

    #[wasm_bindgen]
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();

        ctx.arc(self.x, self.y, self.radius, 0.0, std::f64::consts::PI * 2.0)
            .expect("arc failed while drawing a circle.");

        ctx.set_fill_style(&JsValue::from_str(self.color.as_str()));
        ctx.fill();
    }

    #[wasm_bindgen]
    pub fn get_coordinate(&self) -> Coordinate {
        Coordinate {
            x: self.x.clone(),
            y: self.x.clone(),
        }
    }
}

// impl Move for Circle {
//     fn moving(&mut self, velocity: Velocity) {
//         self.x += velocity.dx;
//         self.y += velocity.dy;
//     }
// }

// impl Draw for Circle {
//     fn draw(&self, ctx: &CanvasRenderingContext2d) {
//         ctx.begin_path();

//         ctx.arc(self.x, self.y, self.radius, 0.0, std::f64::consts::PI * 2.0)
//             .expect("arc failed while drawing a circle.");

//         ctx.set_fill_style(&JsValue::from_str(self.color.as_str()));
//         ctx.fill();
//     }
// }

// impl GetCoordinate for Circle {
//     fn get_coordinate(&self) -> (X, Y) {
//         (self.x, self.y)
//     }
// }
