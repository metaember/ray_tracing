use glam::DVec3;
use rand::prelude::*;

use crate::{
    hittable::Hittable,
    ppm::PPM,
    types::{Color, Point, Ray},
};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const DEFAULT_IMAGE_WIDTH: u32 = 400;
pub const DEFAULT_SAMPLES_PER_PIXEL: u32 = 10;
const MAX_COLOR: u8 = 255;

/// Calculate the image height, and ensure that it's at least 1.
pub fn get_height(image_width: u32, aspect_ratio: f64) -> u32 {
    1.max((image_width as f64 / aspect_ratio) as u32)
}

pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: u32,
    /// Rendered image height in pixel count
    image_height: u32,
    /// Samples per pixel (anti-aliasing)
    pub samples_per_pixel: u32,
    /// Camera center
    center: Point,
    /// Location of pixel (0, 0)
    pixel00_loc: Point,
    /// Offset to pixel to the right
    pixel_delta_u: DVec3,
    /// Offset to pixel below
    pixel_delta_v: DVec3,
}

impl Camera {
    pub fn default() -> Self {
        Camera::new(ASPECT_RATIO, DEFAULT_IMAGE_WIDTH, DEFAULT_SAMPLES_PER_PIXEL)
    }

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let image_height = get_height(image_width, aspect_ratio);
        let center = Point::new(0., 0., 0.);

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

        Self {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc: pixel_00_location,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable, name: &str) {
        let render_fn = |x: f64, y: f64| {
            (0..self.samples_per_pixel)
                .into_iter()
                .map(|_| Camera::ray_color(&self.get_ray(x, y), world))
                .fold(Color::new(0., 0., 0.), |acc, c| acc + c)
                .scale(1. / self.samples_per_pixel as f64)
        };
        let image = PPM::new(self.image_width, self.image_height, MAX_COLOR);
        image.write_fn(&format!("{name}.ppm"), render_fn).unwrap();
    }

    fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
        if let Some(hit) = world.hit(ray, 0., f64::INFINITY) {
            return Color::new_from(0.5 * (1. + hit.normal()));
        }
        // background
        let unit_direction = ray.direction.normalize();
        let vpos = 0.5 * (unit_direction.y + 1.);
        return Color::new_from(Color::new(1., 1., 1.).lerp(*Color::new(0.5, 0.7, 1.), vpos));
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let pixel_center = self.pixel00_loc + x * self.pixel_delta_u + y * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let direction = pixel_sample - self.center;
        Ray::new(self.center, direction)
    }

    /// Sample a random point in a box around the 0,0 pixel
    fn pixel_sample_square(&self) -> Point {
        let mut rng = thread_rng();
        let px = rng.gen_range(-0.5..=0.5);
        let py = rng.gen_range(-0.5..=0.5);
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
}
