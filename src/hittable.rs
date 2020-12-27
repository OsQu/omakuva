use crate::ray;
use crate::vec3::*;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
