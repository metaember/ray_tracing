use glam::DVec3;

use crate::types::{Point, Ray};

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    p: Point,
    normal: DVec3,
    t: f64,
}

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2. * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius as f64 * self.radius as f64;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        }
        let t = (-b - discriminant.sqrt()) / (2. * a);
        if t < t_max && t > t_min {
            let p = r.at(t);
            let normal = (p - self.center) / self.radius as f64;
            return Some(HitRecord { p, normal, t });
        }
        let t = (-b + discriminant.sqrt()) / (2. * a);
        if t < t_max && t > t_min {
            let p = r.at(t);
            let normal = (p - self.center) / self.radius as f64;
            return Some(HitRecord { p, normal, t });
        }
        None
    }
}
