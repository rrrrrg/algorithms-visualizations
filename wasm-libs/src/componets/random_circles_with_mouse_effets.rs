use js_sys::Math::random;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use web_sys::MouseEvent;

use crate::{
    action::Velocity,
    canvas::{self, request_animation_frame, Boundary, Coordinate},
    shapes::circle::Circle,
};

#[wasm_bindgen]
pub fn run_random_circles_with_mouse_effets(document_id: &str, boundary: Boundary) {
    let colors = vec!["#F9EFDB", "#EBD9B4", "#9DBC98", "#638889"];

    let canvas = canvas::canvas(document_id);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut circles: Vec<Circle> = vec![];

    for _ in 0..200 {
        let radius = random() * 20.0;
        let x = random() * boundary.width;
        let y = random() * boundary.height;
        let color = colors[(random() * 4.0) as usize];

        let mut circle = Circle::new(
            color.to_string(),
            radius,
            Coordinate { x, y },
            boundary.clone(),
            None,
        );

        circle.set_velocity(Velocity::get_random_velocity(None));

        circles.push(circle);
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mouse_coords = Rc::new(RefCell::new(Coordinate { x: 0.0, y: 0.0 }));
    // let mouse_coords_clone = mouse_coords.clone();

    // let mouse_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
    //     let x = event.offset_x() as f64;
    //     let y = event.offset_y() as f64;

    //     *mouse_coords_clone.borrow_mut() = Coordinate { x, y };
    // });

    // let _ = canvas
    //     .add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref());

    // mouse_closure.forget();

    *g.borrow_mut() = Some(Closure::new(move || {
        ctx.clear_rect(0.0, 0.0, boundary.width, boundary.height);

        for circle in circles.iter_mut() {
            // circle.mouse_effects(Some(mouse_coords.borrow().clone()));
            circle.moving();
            circle.draw(&ctx);
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
