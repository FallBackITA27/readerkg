use std::path::Path;

use ndarray::Array3;
use rkg_utils::{
    Ghost,
    input_data::{dpad_input::DPadButton, face_input::FaceButton},
};
use video_rs::{Encoder, Time, encode::Settings};

use crate::render_video::draw::{Coordinate, DrawElement, dpad::Dpad, map::DrawController};

pub mod draw;

pub fn render(ghost: &Ghost) {
    const WIDTH: usize = 2000;
    const HEIGHT: usize = 2000;

    video_rs::init().unwrap();
    let settings = Settings::preset_h264_yuv420p(WIDTH, HEIGHT, false);
    let mut encoder =
        Encoder::new(Path::new("test.mp4"), settings).expect("failed to create encoder");

    let accelerator = draw::circle::Circle::new(draw::Coordinate::new(1000, 1000), 100.0);
    let brake = draw::circle::Circle::new(draw::Coordinate::new(800, 1100), 50.0);
    let dpad = Dpad::new(50, Coordinate::new(500, 1000));

    let pixel_map = DrawController::<WIDTH, HEIGHT>::new(accelerator, brake, dpad);

    let duration: Time = Time::from_nth_of_a_second(60);
    let mut position = Time::zero();
    for (frame_number, input) in ghost.input_data().inputs().iter().enumerate() {
        println!("{frame_number} / {}", ghost.input_data().inputs().len());

        let frame = Array3::from_shape_fn((WIDTH, HEIGHT, 3), |(y, x, c)| {
            pixel_map.shape_fn(x, y, c, input)
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
