use glam::DVec3;
use std::ops::Deref;

pub struct Color(DVec3);

impl Deref for Color {
    type Target = DVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(DVec3::new(r, g, b))
    }

    pub fn new_from(v: DVec3) -> Self {
        Self(v)
    }

    pub fn write(self) -> String {
        let ir = (255.999 * self.x) as u32;
        let ig = (255.999 * self.y) as u32;
        let ib = (255.999 * self.z) as u32;
        format!("{} {} {}", ir, ig, ib)
    }
}

pub type Point = DVec3;

pub struct Ray {
    pub origin: Point,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: Point, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
