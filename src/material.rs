use crate::color;
use crate::hittable;
use crate::ray;

pub trait Material {
    /// Returns tuple (attenuation, scattered ray)
    fn scatter(
        &self,
        ray_in: &ray::Ray,
        hit_record: &hittable::HitRecord,
    ) -> (color::Color, ray::Ray);
}
