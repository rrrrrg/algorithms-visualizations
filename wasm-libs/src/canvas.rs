use web_sys::CanvasRenderingContext2d;

pub trait Draw {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
}

pub trait Coordinate {
    fn get_coordinate(&self) -> (f64, f64);
}
