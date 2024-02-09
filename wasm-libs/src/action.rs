#[derive(Clone, Debug)]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

impl Velocity {
    pub fn new(dx: f64, dy: f64) -> Velocity {
        Velocity { dx, dy }
    }

    pub fn get_random_velocity(max: f64) -> Velocity {
        let random_bool_dy = js_sys::Math::random() > 0.5;
        let random_bool_dx = js_sys::Math::random() > 0.5;

        if random_bool_dx && random_bool_dy {
            let dx = js_sys::Math::random() * max * -1.0;
            let dy = js_sys::Math::random() * max * -1.0;

            return Velocity { dx, dy };
        }

        if random_bool_dx {
            let dx = js_sys::Math::random() * max * -1.0;
            let dy = js_sys::Math::random() * max;

            return Velocity { dx, dy };
        }

        if random_bool_dy {
            let dx = js_sys::Math::random() * max;
            let dy = js_sys::Math::random() * max * -1.0;

            return Velocity { dx, dy };
        }

        let dx = js_sys::Math::random() * max;
        let dy = js_sys::Math::random() * max;

        return Velocity { dx, dy };
    }
}

pub trait SetVelocity {
    fn set_velocity(&mut self, velocity: Velocity);
}
