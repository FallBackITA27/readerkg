use std::path::Path;

use ndarray::Array3;
use video_rs::{Encoder, Time, encode::Settings};

use crate::render_video::draw::{Coordinate, DrawElement};

pub mod draw;

pub fn render() {
    const WIDTH: usize = 2000;
    const HEIGHT: usize = 2000;
    video_rs::init().unwrap();
    let settings = Settings::preset_h264_yuv420p(WIDTH, HEIGHT, false);
    let mut encoder =
        Encoder::new(Path::new("test.mp4"), settings).expect("failed to create encoder");

    let circle = draw::circle::Circle::new(draw::Coordinate::new(200, 200), 30.0);
    let path = draw::path::Path::new([
        draw::Coordinate::new(100, 100),
        draw::Coordinate::new(50, 150),
        draw::Coordinate::new(100, 200),
        draw::Coordinate::new(200, 200),
        draw::Coordinate::new(200, 100),
        draw::Coordinate::new(100, 100),
    ]);

    let duration: Time = Time::from_nth_of_a_second(60);
    let mut position = Time::zero();
    for i in 0..256 {
        let frame = Array3::from_shape_fn((WIDTH, HEIGHT, 3), |(y, x, c)| {
            let coordinate = Coordinate::new((x - i) as i32, y as i32);
            match (
                circle.coordinate_in_path(coordinate, 2),
                path.coordinate_in_path(coordinate, 2),
                path.coordinate_inside(coordinate) || circle.coordinate_inside(coordinate),
            ) {
                (_, _, true) => [0, 0, 0xff][c],
                (true, true, _) => 0xff,
                (true, false, _) => [0xff, 0, 0][c],
                (false, true, _) => [0, 0xff, 0][c],
                (false, false, _) => 0x00,
            }
        });

        encoder
            .encode(&frame, position)
            .expect("failed to encode frame");

        // Update the current position and add the inter-frame duration to it.
        position = position.aligned_with(duration).add();
    }

    encoder.finish().expect("failed to finish encoder");
}
