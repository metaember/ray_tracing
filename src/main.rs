pub mod ppm;

use crate::ppm::PPM;

const IMAGE_WIDT: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: u8 = 255;

fn gradient(x: f64, y: f64) -> glam::DVec3 {
    glam::DVec3::new(x, y, 0.)
}

fn main() {
    let image = PPM::new(IMAGE_WIDT, IMAGE_HEIGHT, MAX_COLOR);
    image.write_fn("gradient.ppm", gradient).unwrap();
}
