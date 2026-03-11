pub mod circle;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn check_point_overlap(self, coordinate: Coordinate, width: i32) -> bool {
        let width = width / 2;
        let range = -width..=width;

        range.contains(&(self.x - coordinate.x)) && range.contains(&(self.y - coordinate.y))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Path<const N: usize>([Coordinate; N]);
impl<const N: usize> Path<N> {
    pub fn new(v: [Coordinate; N]) -> Self {
        Self(v)
    }

    pub fn is_closed(&self) -> bool {
        self.0[0] == self.0[N-1]
    }

    pub fn coordinate_within(&self, coordinate: Coordinate) -> bool {
        let internal_sum: f64 = 180.0 * ((N as f64) - 2.0);
        if !self.is_closed() {
            return false;
        }
        for idx in 1..N {
            let coord1 = self.0[idx-1];
            let coord2 = self.0[idx];


            
        }
        false
    }

    pub fn check_point_on_path(&self, coordinate: Coordinate, width: i32) -> bool {
        let (x0, y0) = (coordinate.x as f64, coordinate.y as f64);
        for idx in 1..N {
            let (x1, y1) = (self.0[idx - 1].x, self.0[idx - 1].y);
            let (x2, y2) = (self.0[idx].x, self.0[idx].y);

            {
                let x_max = std::cmp::max(x1, x2);
                let x_min = match x_max == x1 {
                    true => x2,
                    false => x1,
                };

                let y_max = std::cmp::max(y1, y2);
                let y_min = match y_max == y1 {
                    true => y2,
                    false => y1,
                };

                if !(x_min - width..=x_max + width).contains(&coordinate.x)
                    || !(y_min - width..=y_max + width).contains(&coordinate.y)
                {
                    continue;
                }
            }

            let (x1, y1) = (x1 as f64, y1 as f64);
            let (x2, y2) = (x2 as f64, y2 as f64);

            let distance = (((y2 - y1) * x0) - ((x2 - x1) * y0) + (x2 * y1) - (y2 * x1)).abs()
                / ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
            if (distance as i32) <= width {
                return true;
            }
        }

        false
    }
}
