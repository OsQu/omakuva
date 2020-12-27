use crate::ray;
use crate::vec3::*;

pub struct HitRecord<'a> {
    pub point: &'a Point3,
    pub normal: &'a Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit<'a>(ray: &'a ray::Ray, t_min: f32, t_max: f32) -> HitRecord<'a>;
}
