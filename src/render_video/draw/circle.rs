use crate::render_video::draw::Coordinate;

pub struct Circle {
    center_coords: Coordinate,
    radius: f64,
    line_width: u8,
}

impl Circle {
    pub const fn new(center_coords: Coordinate, radius: f64, line_width: u8) -> Self {
        Self {
            center_coords,
            radius,
            line_width,
        }
    }

    pub fn check_point_on_line(&self, coord: Coordinate) -> bool {
        let (x, y) = (
            coord.x - self.center_coords.x,
            coord.y - self.center_coords.y,
        );
        let (x, y) = (x as f64, y as f64);
        let (x, y) = (x / self.radius, y / self.radius);
        let angle = x.atan2(y);

        self.calc_point(angle)
            .check_point_overlap(coord, self.line_width as i32)
    }

    pub fn calc_point(&self, angle: f64) -> Coordinate {
        let (sin, cos) = angle.sin_cos();
        let (sin, cos) = (
            sin * self.radius + (self.center_coords.x as f64),
            cos * self.radius + (self.center_coords.y as f64),
        );
        Coordinate::new(sin as i32, cos as i32)
    }

    pub fn calc_points(&self) -> Vec<Coordinate> {
        const PRECISION: f64 = 1000000.0;
        (0..=((std::f64::consts::PI * PRECISION) as u32))
            .map(|v| v as f64)
            .map(|v| v * PRECISION)
            .map(|v| self.calc_point(v))
            .collect()
    }
}
