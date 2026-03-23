use crate::render_video::draw::{Coordinate, path::Path};

pub struct Dpad {
    pub up: Path<6>,
    pub down: Path<6>,
    pub left: Path<6>,
    pub right: Path<6>,
}

impl Dpad {
    pub const fn new(scale: i32, center: Coordinate) -> Self {
        const SIDE_LEN: i32 = 3;

        let up = Path::new([
            center,                                                         // Tip
            Coordinate::new(center.x + scale, center.y - scale),            // Bottom Right
            Coordinate::new(center.x + scale, center.y - scale * SIDE_LEN), // Top Right
            Coordinate::new(center.x - scale, center.y - scale * SIDE_LEN), // Top Left
            Coordinate::new(center.x - scale, center.y - scale),            // Bottom Left
            center,                                                         // Tip
        ]);
        let right = Path::new([
            center,                                                         // Tip
            Coordinate::new(center.x + scale, center.y - scale),            // Top Left
            Coordinate::new(center.x + scale * SIDE_LEN, center.y - scale), // Top Right
            Coordinate::new(center.x + scale * SIDE_LEN, center.y + scale), // Bottom Right
            Coordinate::new(center.x + scale, center.y + scale),            // Bottom Left
            center,                                                         // Tip
        ]);
        let down = Path::new([
            center,                                                         // Tip
            Coordinate::new(center.x + scale, center.y + scale),            // Top Right
            Coordinate::new(center.x + scale, center.y + scale * SIDE_LEN), // Bottom Right
            Coordinate::new(center.x - scale, center.y + scale * SIDE_LEN), // Bottom Left
            Coordinate::new(center.x - scale, center.y + scale),            // Top Left
            center,                                                         // Tip
        ]);
        let left = Path::new([
            center,                                                         // Tip
            Coordinate::new(center.x - scale, center.y + scale),            // Bottom Right
            Coordinate::new(center.x - scale * SIDE_LEN, center.y + scale), // Bottom Left
            Coordinate::new(center.x - scale * SIDE_LEN, center.y - scale), // Top Left
            Coordinate::new(center.x - scale, center.y - scale),            // Top Right
            center,                                                         // Tip
        ]);
        Self {
            up,
            right,
            left,
            down,
        }
    }
}
