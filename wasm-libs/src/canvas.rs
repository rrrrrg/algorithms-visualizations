use crate::shapes::circle::{Circle, MovingCircle};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

#[wasm_bindgen]
impl Velocity {
    pub fn new(dx: f64, dy: f64) -> Velocity {
        Velocity { dx, dy }
    }
}

#[wasm_bindgen]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Coordinate {
    pub fn new(x: f64, y: f64) -> Coordinate {
        Coordinate { x, y }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let colors = ["#272F32", "#9DBDC6", "#FF3D2E", "#DAEAEF"];

    let mut circle = MovingCircle::new(
        Coordinate::new(30.0, 80.0),
        Circle::new(10.0),
        Velocity::new(2.0, 3.0),
    );
    context.clear_rect(0.0, 0.0, 800.0, 800.0);
    circle.moving();
    context.begin_path();
    context
        .arc(
            circle.get_coordinate().x,
            circle.get_coordinate().y,
            circle.get_radius(),
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .expect("arc failed");

    context.set_fill_style(&JsValue::from_str(colors[0]));
    context.fill();
}
