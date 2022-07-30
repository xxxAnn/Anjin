struct Coordinates {
  x: f32,
  y: f32,
}

struct Curve {
  coords: Vec<Coordinates>
}

impl Curve {
  fn intersects(&self) -> bool {
    self.coords.map(|c, i| {
      
    })
  }
}