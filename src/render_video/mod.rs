use std::path::Path;

use ndarray::Array3;
use rkg_utils::{Ghost, input_data::{dpad_input::DPadButton, face_input::FaceButton}};
use video_rs::{Encoder, Time, encode::Settings};

use crate::render_video::draw::{Coordinate, DrawElement};

pub mod draw;

pub fn render(ghost: &Ghost) {
    const WIDTH: usize = 2000;
    const HEIGHT: usize = 2000;

    video_rs::init().unwrap();
    let settings = Settings::preset_h264_yuv420p(WIDTH, HEIGHT, false);
    let mut encoder =
        Encoder::new(Path::new("test.mp4"), settings).expect("failed to create encoder");

    const OUTLINE_WIDTH: i32 = 5;

    let accelerator = draw::circle::Circle::new(draw::Coordinate::new(1000, 1000), 100.0);
    let brake = draw::circle::Circle::new(draw::Coordinate::new(800, 1100), 50.0);
    let dpad_up = draw::path::Path::new([
        Coordinate::new(200, 200),
        Coordinate::new(250, 150),
        Coordinate::new(250, 100),
        Coordinate::new(150, 100),
        Coordinate::new(150, 150),
        Coordinate::new(200, 200),
    ]);

    const BLACK_PX: u8 = 0x00;
    const WHITE_PX: u8 = 0xFF;
    const ACCEL_PX: u8 = 0x01;
    const BRAKE_PX: u8 = 0x02;
    const DPADUP_PX: u8 = 0xA1;
    const DPADRIGHT_PX: u8 = 0xA2;
    const DPADDOWN_PX: u8 = 0xA3;
    const DPADLEFT_PX: u8 = 0xA4;

    let mut pixel_map = [[0u8; WIDTH]; HEIGHT];
    for row_idx in 0..HEIGHT {
        for column_idx in 0..WIDTH {
            let coordinate = Coordinate::new(column_idx as i32, row_idx as i32);
            pixel_map[row_idx][column_idx] = if accelerator.coordinate_in_path(coordinate, OUTLINE_WIDTH) || brake.coordinate_in_path(coordinate, OUTLINE_WIDTH)  || dpad_up.coordinate_in_path(coordinate, OUTLINE_WIDTH){
                WHITE_PX
            } else if accelerator.coordinate_inside(coordinate) {
                ACCEL_PX
            } else if brake.coordinate_inside(coordinate) {
               BRAKE_PX 
            }else if dpad_up.coordinate_inside(coordinate) {
               DPADUP_PX
            }else {
                BLACK_PX
            };
        }
    }
    let pixel_map = pixel_map;

    let duration: Time = Time::from_nth_of_a_second(60);
    let mut position = Time::zero();
    for (frame_number, input) in ghost.input_data().inputs().iter().enumerate() {
        println!("{frame_number} / {}", ghost.input_data().inputs().len());

        let frame = Array3::from_shape_fn((WIDTH, HEIGHT, 3), |(y, x, c)| {
            match pixel_map[y][x] {
                WHITE_PX => 0xFF,
                ACCEL_PX if input.face_buttons().contains(&FaceButton::Accelerator) => [0, 0xFF, 0][c],
                BRAKE_PX if input.face_buttons().contains(&FaceButton::Brake) => [0xFF, 0, 0][c],
                DPADUP_PX if input.dpad_button() == DPadButton::Up => 0xFF,
                _ => 0x00
            }
        });

        encoder
            .encode(&frame, position)
            .expect("failed to encode frame");

        // Update the current position and add the inter-frame duration to it.
        for _ in 0..=input.frame_duration() {
            position = position.aligned_with(duration).add();
        }
    }

    encoder.finish().expect("failed to finish encoder");
}
