use crate::{
    canvas::{self, Drawable},
    shapes::square::Square,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw_a_square() {
    let square = Square::new(100.0);

    let canvas = canvas::canvas("square-canvas");

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.set_stroke_style(&JsValue::from_str("#000000"));

    ctx.save();

    let _ = ctx.translate(100.0, 100.0);

    square.draw(&ctx);

    ctx.restore();
}
