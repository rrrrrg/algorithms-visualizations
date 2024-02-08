#[derive(Clone, Debug)]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

impl Velocity {
    pub fn new(dx: f64, dy: f64) -> Velocity {
        Velocity { dx, dy }
    }

    pub fn get_random_velocity(max_velocity: Option<Velocity>) -> Velocity {
        let random_bool = js_sys::Math::random() > 0.5;

        if let Some(max_velocity) = max_velocity {
            if random_bool {
                let dx = js_sys::Math::random() * max_velocity.dx * -1.0;
                let dy = js_sys::Math::random() * max_velocity.dy * -1.0;

                return Velocity { dx, dy };
            }
            let dx = js_sys::Math::random() * max_velocity.dx;
            let dy = js_sys::Math::random() * max_velocity.dy;

            return Velocity { dx, dy };
        }

        if random_bool {
            let dx = js_sys::Math::random() * -2.0;
            let dy = js_sys::Math::random() * -2.0;

            return Velocity { dx, dy };
        }
        let dx = js_sys::Math::random() * 2.0;
        let dy = js_sys::Math::random() * 2.0;

        Velocity { dx, dy }
    }
}

pub trait SetVelocity {
    fn set_velocity(&mut self, velocity: Velocity);
}
