use std::fmt::Write;

use rkg_utils::input_data::{dpad_input::DPadButton, face_input::FaceButton, input::Input};

use crate::render_video::draw::{Coordinate, DrawElement, circle::Circle, dpad::Dpad, path::Path};

#[derive(Default, Clone, Copy)]
enum ItemMap {
    #[default]
    Background,
    Stroke,
    Accelerate,
    Brake,
    DpadUp,
    DpadRight,
    DpadDown,
    DpadLeft,
}

pub struct DrawController<const WIDTH: usize, const HEIGHT: usize> {
    symbols: [[ItemMap; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> DrawController<WIDTH, HEIGHT> {
    pub fn new(accelerator: Circle, brake: Circle, dpad: Dpad) -> Self {
        const OUTLINE_WIDTH: i32 = 5;

        let mut symbols = [[ItemMap::Background; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let coordinate = Coordinate::new(x as i32, y as i32);
                symbols[x][y] = if accelerator.coordinate_in_path(coordinate, OUTLINE_WIDTH)
                    || brake.coordinate_in_path(coordinate, OUTLINE_WIDTH)
                    || dpad.up.coordinate_in_path(coordinate, OUTLINE_WIDTH)
                    || dpad.right.coordinate_in_path(coordinate, OUTLINE_WIDTH)
                    || dpad.down.coordinate_in_path(coordinate, OUTLINE_WIDTH)
                    || dpad.left.coordinate_in_path(coordinate, OUTLINE_WIDTH)
                {
                    ItemMap::Stroke
                } else if accelerator.coordinate_inside(coordinate) {
                    ItemMap::Accelerate
                } else if brake.coordinate_inside(coordinate) {
                    ItemMap::Brake
                } else if dpad.up.coordinate_inside(coordinate) {
                    ItemMap::DpadUp
                } else if dpad.right.coordinate_inside(coordinate) {
                    ItemMap::DpadRight
                } else if dpad.down.coordinate_inside(coordinate) {
                    ItemMap::DpadDown
                } else if dpad.left.coordinate_inside(coordinate) {
                    ItemMap::DpadLeft
                } else {
                    ItemMap::Background
                };
            }
        }

        Self { symbols }
    }
    pub fn shape_fn(&self, x: usize, y: usize, color_idx: usize, input: &Input) -> u8 {
        if x > WIDTH || y > HEIGHT {
            return 0x00;
        }

        match self.symbols[x][y] {
            ItemMap::Background => 0x00,
            ItemMap::Stroke => 0xFF,
            ItemMap::Accelerate if input.face_buttons().contains(&FaceButton::Accelerator) => {
                [0x00, 0xFF, 0x00][color_idx]
            }
            ItemMap::Brake if input.face_buttons().contains(&FaceButton::Brake) => {
                [0xFF, 0x00, 0x00][color_idx]
            }
            ItemMap::DpadUp if input.dpad_button() == DPadButton::Up => 0xFF,
            ItemMap::DpadRight if input.dpad_button() == DPadButton::Right => 0xFF,
            ItemMap::DpadDown if input.dpad_button() == DPadButton::Down => 0xFF,
            ItemMap::DpadLeft if input.dpad_button() == DPadButton::Left => 0xFF,
            ItemMap::Accelerate
            | ItemMap::Brake
            | ItemMap::DpadUp
            | ItemMap::DpadRight
            | ItemMap::DpadDown
            | ItemMap::DpadLeft => 0x00,
        }
    }
}
