pub trait Coords2D {
    fn distance_from(&self, c: &Self) -> f32 {
        f32::sqrt(f32::powi(c.x() - self.x(), 2)+f32::powi(c.y() - self.y(), 2))
    }
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

pub struct Coordinates2D {
    x: f32,
    y: f32
}

impl Coords2D for Coordinates2D {
    fn x(&self) -> f32 {
        return self.x
    }
    fn y(&self) -> f32 {
        return self.y
    }
}

impl Coordinates2D {
    pub fn new<T>(x: T, y: T) -> Coordinates2D
    where T: Into<f32> {
        return Coordinates2D { x: x.into(), y: y.into() }
    }
}
