use crate::color;
use crate::hittable;
use crate::ray;

pub trait Material {
    /// Returns tuple (attenuation, scattered ray)
    fn scatter<'a>(
        &self,
        ray_in: &ray::Ray,
        hit_record: &'a hittable::HitRecord,
    ) -> Option<(color::Color, ray::Ray<'a>)>;
}
