pub mod circle;
pub mod path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_f64(self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }

    pub fn check_point_overlap(self, coordinate: Coordinate, width: i32) -> bool {
        let width = width / 2;
        let range = -width..=width;

        range.contains(&(self.x - coordinate.x)) && range.contains(&(self.y - coordinate.y))
    }

    pub fn get_distance(self, rhs: Coordinate) -> f64 {
        let (x1, y1) = self.as_f64();
        let (x2, y2) = rhs.as_f64();
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
}

pub trait DrawElement {
    fn coordinate_in_path(&self, coordinate: Coordinate, width: i32) -> bool;
    fn coordinate_inside(&self, coordinate: Coordinate) -> bool;
}
