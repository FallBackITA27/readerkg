use std::path::Path;

use ndarray::Array3;
use video_rs::{Encoder, Time, encode::Settings};

use crate::render_video::draw::Coordinate;

pub mod draw;

pub fn render() {
    video_rs::init().unwrap();
    let settings = Settings::preset_h264_yuv420p(400, 400, false);
    let mut encoder =
        Encoder::new(Path::new("test.mp4"), settings).expect("failed to create encoder");

    let circle = draw::circle::Circle::new(draw::Coordinate::new(200, 200), 30.0, 2);
    let path = draw::Path::new([
        draw::Coordinate::new(200, 200),
        draw::Coordinate::new(100, 200),
        draw::Coordinate::new(200, 100),
        draw::Coordinate::new(100, 100),
        draw::Coordinate::new(200, 200),
    ]);

    let duration: Time = Time::from_nth_of_a_second(60);
    let mut position = Time::zero();
    for i in 0..256 {
        let frame = Array3::from_shape_fn((400, 400, 3), |(y, x, c)| {
            let coordinate = Coordinate::new((x - i) as i32, y as i32);
            match (
                circle.check_point_on_line(coordinate),
                path.check_point_on_path(coordinate, 2),
            ) {
                (true, true) => 0xff,
                (true, false) => [0xff, 0, 0][c],
                (false, true) => [0, 0xff, 0][c],
                (false, false) => 0x00,
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
