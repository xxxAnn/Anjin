#[derive(Debug)]
struct Coordinates {
  x: f32,
  y: f32
}

#[derive(Debug)]
struct Curve {
  coords: Vec<Coordinates>
}

impl Curve {
  pub fn intersects(&self) -> bool {
    for i in 0..(self.coords.len()) {
      for j in i+1..(self.coords.len()) {
        let a: f32 = self.coords[i].x - self.coords[j].x;
        let b: f32 = self.coords[i].y - self.coords[j].y;

        if a == 0. && b == 0. {
          return true
        }
      }
    }
    false
  }
}

fn main() {
  let curve: Curve = {Curve {coords: vec![{Coordinates{x: 0., y: 2.}}, {Coordinates{x: 0., y: 2.}}, {Coordinates{x: 1., y: 3.}}]}};
  
  println!("{:?}", curve.intersects());
}