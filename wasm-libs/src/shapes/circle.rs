use crate::{
    action::{SetVelocity, Velocity},
    canvas::{Boundary, Coordinate, Drawable, MouseMoveEffects},
};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Circle {
    color: String,
    init_radius: f64,
    radius: f64,
    coordinate: Coordinate,
    boundary: Boundary,
    velocity: Velocity,
}

impl Circle {
    pub fn new(
        color: String,
        radius: f64,
        coordinate: Coordinate,
        boundary: Boundary,
        velocity: Option<Velocity>,
    ) -> Circle {
        Circle {
            color,
            init_radius: radius,
            radius,
            coordinate,
            boundary,
            velocity: velocity.unwrap_or(Velocity { dx: 0.0, dy: 0.0 }),
        }
    }

    pub fn moving(&mut self) {
        self.coordinate.x += self.velocity.dx;
        self.coordinate.y += self.velocity.dy;

        // Bounce when hit the boundary
        let is_x_coord_hit_boundary = self.coordinate.x + self.radius > self.boundary.width
            || self.coordinate.x - self.radius < 0.0;

        let is_y_coord_hit_boundary = self.coordinate.y + self.radius > self.boundary.height
            || self.coordinate.y - self.radius < 0.0;

        if is_x_coord_hit_boundary {
            self.velocity.dx = self.velocity.dx * -1.0;
        }

        if is_y_coord_hit_boundary {
            self.velocity.dy = self.velocity.dy * -1.0;
        }
    }
}

impl MouseMoveEffects for Circle {
    fn mouse_move_effects(&mut self, mouse_coordinate: &Coordinate) {
        let max_radius = 35.0;
        let x_distance = mouse_coordinate.x - self.coordinate.x;
        let y_distance = mouse_coordinate.y - self.coordinate.y;

        if x_distance < 50.0
            && x_distance > -50.0
            && self.radius < max_radius
            && y_distance < 50.0
            && y_distance > -50.0
        {
            self.radius += 2.0;
        } else if (x_distance >= 50.0 && self.init_radius < self.radius)
            || (x_distance <= -50.0 && self.init_radius < self.radius)
            || (y_distance >= 50.0 && self.init_radius < self.radius)
            || (y_distance <= -50.0 && self.init_radius < self.radius)
        {
            self.radius -= 2.0;
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();

        if let Ok(_) = ctx.arc(
            self.coordinate.x,
            self.coordinate.y,
            self.radius,
            0.0,
            std::f64::consts::PI * 2.0,
        ) {
            ctx.set_fill_style(&JsValue::from_str(self.color.as_str()));
            ctx.fill();
        }
    }
}

impl SetVelocity for Circle {
    fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}
