use crate::color;
use crate::hittable;
use crate::material;
use crate::ray;
use crate::vec3::*;

pub struct Lambertian {
    albedo: color::Color,
}

impl Lambertian {
    pub fn new(albedo: color::Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl material::Material for Lambertian {
    fn scatter<'a>(
        &self,
        _ray_in: &ray::Ray,
        hit_record: &'a hittable::HitRecord,
    ) -> Option<(color::Color, ray::Ray<'a>)> {
        let mut scatter_direction = &hit_record.normal + &Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal.clone()
        }
        let scattered = ray::Ray {
            orig: &hit_record.point,
            dir: scatter_direction,
        };

        return Some((self.albedo.clone(), scattered));
    }
}
