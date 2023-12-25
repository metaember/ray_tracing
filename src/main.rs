pub mod ppm;
pub mod types;

use crate::ppm::PPM;
use crate::types::Color;

const IMAGE_WIDT: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: u8 = 255;

fn gradient(x: f64, y: f64) -> Color {
    Color::new(0., x, y)
}

fn main() {
    let image = PPM::new(IMAGE_WIDT, IMAGE_HEIGHT, MAX_COLOR);
    image.write_fn("gradient.ppm", gradient).unwrap();
}
