pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const DEFAULT_IMAGE_WIDTH: u32 = 400;

/// Calculate the image height, and ensure that it's at least 1.
pub fn get_height(image_width: u32, aspect_ratio: f64) -> u32 {
    1.max((image_width as f64 / aspect_ratio) as u32)
}
