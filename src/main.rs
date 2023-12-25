pub mod camera;
pub mod ppm;
pub mod types;

use glam::DVec3;

use crate::ppm::PPM;
use crate::types::{Color, Point, Ray};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: u8 = 255;

fn generate_gradient() {
    let image = PPM::new(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR);
    let gradient = |x: f64, y: f64| {
        let r = 0.;
        let g = x / IMAGE_WIDTH as f64;
        let b = y / IMAGE_HEIGHT as f64;
        Color::new(r, g, b)
    };

    image.write_fn("gradient.ppm", gradient).unwrap();
}

// ray tracing proper

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let vpos = 0.5 * (unit_direction.y + 1.);
    return Color::new_from(Color::new(1., 1., 1.).lerp(*Color::new(0.5, 0.7, 1.), vpos));
}

fn main() {
    // Image setup
    let aspect_ratio = camera::ASPECT_RATIO;
    let image_width = camera::DEFAULT_IMAGE_WIDTH;
    let image_height = camera::get_height(image_width, aspect_ratio);

    // Camera setup
    // distance from the camera to the viewport
    let focal_length = 1.0;
    // viewport will be real valued
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point::new(0., 0., 0.);

    // camera is located at (0, 0, 0)
    // with y axis pointing up, x axis pointing right,
    // and negative z axis pointing to the viewport
    // (right handed rule)
    // so the viewport center is at (0, 0, -focal_length)

    // let u be the vector along the viewport horizontally left to right
    // v be the vector along the viewport vertically top to bottom
    let viewport_u = DVec3::new(viewport_width, 0., 0.);
    let viewport_v = DVec3::new(0., -viewport_height, 0.);

    // pixels are squares on the grid, offset from the edbge by half a pixel width
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // the bottom left corner of the viewport is at
    let viewport_center = camera_center - DVec3::new(0., 0., focal_length);
    let viewport_upper_left = viewport_center - viewport_u / 2. - viewport_v / 2.;
    // coord of the "center" of the top left pixel at (0, 0) in the viewport coords
    let pixel_00_location = viewport_upper_left + pixel_delta_u / 2. - pixel_delta_v / 2.;

    // Render
    let render_fn = |x: f64, y: f64| {
        let pixel_center = pixel_00_location + x * pixel_delta_u + y * pixel_delta_v;
        let direction = pixel_center - camera_center;
        let ray = Ray::new(camera_center, direction);
        ray_color(&ray)
    };

    let image = PPM::new(image_width, image_height, MAX_COLOR);
    image.write_fn("listing_9.ppm", render_fn).unwrap();
}
