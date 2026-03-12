use crate::render_video::draw::{Coordinate, DrawElement};


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Path<const N: usize>([Coordinate; N]);
impl<const N: usize> Path<N> {
    pub fn new(v: [Coordinate; N]) -> Self {
        Self(v)
    }

    pub fn is_closed(&self) -> bool {
        self.0[0] == self.0[N - 1]
    }

    pub fn bounding_box(self) -> (Coordinate, Coordinate) {
        let (mut max_x, mut min_x, mut max_y, mut min_y) = (0, i32::MAX, 0, i32::MAX);
        for idx in 0..N {
            let coordinates = self.0[idx];
            let (x, y) = (coordinates.x, coordinates.y);
            max_x = std::cmp::max(max_x, x);
            min_x = std::cmp::min(min_x, x);
            max_y = std::cmp::max(max_y, y);
            min_y = std::cmp::min(min_y, y);
        }

        (Coordinate::new(min_x, min_y), Coordinate::new(max_x, max_y))
    }

}

impl<const N: usize> DrawElement for Path<N> {
    fn coordinate_in_path(&self, coordinate: Coordinate, width: i32) -> bool {
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

    fn coordinate_inside(&self, coordinate: Coordinate) -> bool {
        if !self.is_closed() {
            return false;
        }

        let (top_left, bottom_right) = self.bounding_box();
        if top_left.x > coordinate.x
            || top_left.y > coordinate.y
            || bottom_right.x < coordinate.x
            || bottom_right.y < coordinate.y
        {
            return false;
        }

        let mut counter = 0;
        let mut skip = 0;
        for x in 0..=coordinate.x {
            if skip > 0 {
                skip -= 1;
            }
            let new_coordinate = Coordinate::new(x, coordinate.y);
            if self.coordinate_in_path(new_coordinate, 0) {
                if skip == 0 {
                    counter += 1u16;
                }

                skip = 10;
            }
        }

        !counter.is_multiple_of(2)
    }

}


