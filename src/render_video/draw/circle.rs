use crate::render_video::draw::{Coordinate, DrawElement};

pub struct Circle {
    center_coords: Coordinate,
    radius: f64,
}

impl Circle {
    pub const fn new(center_coords: Coordinate, radius: f64) -> Self {
        Self {
            center_coords,
            radius,
        }
    }

    pub fn calc_point(&self, angle: f64) -> Coordinate {
        let (sin, cos) = angle.sin_cos();
        let (sin, cos) = (
            sin * self.radius + (self.center_coords.x as f64),
            cos * self.radius + (self.center_coords.y as f64),
        );
        Coordinate::new(sin as i32, cos as i32)
    }
}

impl DrawElement for Circle {
    fn coordinate_inside(&self, coordinate: Coordinate) -> bool {
        self.radius > coordinate.get_distance(self.center_coords)
        
    }
    fn coordinate_in_path(&self, coordinate: Coordinate, width: i32) -> bool {
        let (x, y) = (
            coordinate.x - self.center_coords.x,
            coordinate.y - self.center_coords.y,
        );
        let (x, y) = (x as f64, y as f64);
        let (x, y) = (x / self.radius, y / self.radius);
        let angle = x.atan2(y);

        self.calc_point(angle)
            .check_point_overlap(coordinate, width)
    }
}
