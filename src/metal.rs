use crate::color;
use crate::hittable;
use crate::material;
use crate::ray;
use crate::vec3::*;

pub struct Metal {
    albedo: color::Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: color::Color, mut fuzz: f32) -> Metal {
        fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl material::Material for Metal {
    fn scatter<'a>(
        &self,
        ray_in: &ray::Ray<'_>,
        hit_record: &'a hittable::HitRecord,
    ) -> std::option::Option<(Vec3, ray::Ray<'a>)> {
        let reflected = ray_in.dir.unit_vector().reflect(&hit_record.normal);
        let scattered = ray::Ray {
            orig: &hit_record.point,
            dir: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        };

        if scattered.dir.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}
