use crate::material;
use crate::ray;
use crate::vec3::*;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn material::Material,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
