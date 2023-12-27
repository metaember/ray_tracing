use glam::DVec3;

use crate::types::{Point, Ray};

/// A hittable object, ie, one that can be hit by a ray
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// A collection of hittable objects
/// A hit is determined by the closest hit object
pub struct Hittables {
    hittables: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }

    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
    }
}

impl Hittable for Hittables {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for hittable in &self.hittables {
            if let Some(record) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t();
                hit_record = Some(record);
            }
        }
        hit_record
    }
}

/// A record of a hit
/// Contains the point of intersection, the normal at the point of intersection,
pub struct HitRecord {
    p: Point,
    normal: DVec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point, normal: DVec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
        }
    }

    pub fn new_with_face_normal(p: Point, outward_normal: DVec3, t: f64, r: &Ray) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.;
        Self {
            p,
            normal: outward_normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> DVec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> Point {
        self.p
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: DVec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
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
        let half_b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius as f64 * self.radius as f64;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let t = (-half_b - discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius as f64;
            return Some(HitRecord::new_with_face_normal(p, outward_normal, t, r));
        }
        let t = (-half_b + discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius as f64;
            return Some(HitRecord::new_with_face_normal(p, outward_normal, t, r));
        }
        None
    }
}
