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

#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Coordinate {
        Coordinate { x, y }
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn canvas(element_id: &str) -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id(element_id)
        .expect("should have canvas on document")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn request_animation_frame(cb: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub trait Drawable {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

pub trait MouseMoveEffects<T> {
    fn mouse_move_effects(&mut self, mouse_coordinate: &Coordinate) -> T;
}

pub trait MouseDownEffects<T> {
    fn mouse_down_effects(&mut self, mouse_coordinate: &Coordinate) -> T;
}
