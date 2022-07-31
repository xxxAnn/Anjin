use super::coords::Coords2D;

const DISTANCE_MIN: f32 = 20.;

pub struct UserDrawnLine<T>
where T: Coords2D {
    distance: f32,
    _raw: Vec<T>,
    started: bool,
    closed: bool
}

impl<T> UserDrawnLine<T>
where T: Coords2D {
    pub fn new_point(&mut self, coords: T) {
        if !(self.started) {
            self.started = true;
        } else {
            match self._raw.last() {
                Some(last_coords) => {
                    self.distance += coords.distance_from(last_coords);
                },
                None => {
                    panic!("The line's raw coordinates list was empty, but started was set to true.")
                }
            }
            
        }
        if self.distance > DISTANCE_MIN {
            for o in self._raw.iter() {
                if o.x() == coords.x() && o.y() == coords.y() {
                    self.closed = true;
                }
            }
        }
        self._raw.push(coords);
    }
    pub fn new() -> UserDrawnLine<T> {
        UserDrawnLine {
            distance: 0.,
            _raw: Vec::new(),
            closed: false,
            started: false
        }
    }
}