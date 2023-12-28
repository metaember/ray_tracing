pub mod camera;
pub mod hittable;
pub mod ppm;
pub mod types;

use hittable::Hittables;

use crate::camera::Camera;
use crate::hittable::Sphere;
use crate::types::Point;

pub fn main(name: &str) {
    // Build world
    let mut world = Hittables::new();
    world.push(Box::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

    // Camera setup
    let camera = Camera::new(16. / 9., 400);
    camera.render(&world, name);
}
