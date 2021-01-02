use crate::color;
use crate::hittable;
use crate::material;
use crate::ray;
use crate::vec3::*;
use rand::prelude::*;

pub struct Dielectric {
    ir: f32, // Index of Refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cos: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance
        let mut r0 = (1_f32 - ref_idx) / (1_f32 + ref_idx);
        r0 = r0 * r0;

        return r0 + (1_f32 - r0) * (1_f32 - cos).powi(5);
    }
}

impl material::Material for Dielectric {
    fn scatter<'a>(
        &self,
        ray_in: &ray::Ray<'_>,
        hit_record: &'a hittable::HitRecord,
    ) -> std::option::Option<(Vec3, ray::Ray<'a>)> {
        let attenuation = color::Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.dir.unit_vector();
        let cos_theta = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_retract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();

        let direction =
            if cannot_retract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction.reflect(&hit_record.normal)
            } else {
                unit_direction.refract(&hit_record.normal, refraction_ratio)
            };

        let scattered = ray::Ray {
            orig: &hit_record.point,
            dir: direction,
        };

        return Some((attenuation, scattered));
    }
}
