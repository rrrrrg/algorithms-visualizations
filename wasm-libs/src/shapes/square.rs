use crate::canvas::Drawable;

#[derive(Clone, Debug)]
pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Drawable for Square {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.rect(0.0, 0.0, self.side, self.side);
    }
}
