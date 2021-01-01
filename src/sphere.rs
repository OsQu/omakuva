use crate::hittable;
use crate::ray;
use crate::vec3::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord> {
        // Calculate discriminant from ray-sphere intersection
        let oc = ray.orig - &self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        // Discriminant: b^2 - 4ac: == 0 -> 1 roots, > 0 -> 1 root
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (&point - &self.center) / self.radius;

        let front_face = ray.dir.dot(&outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -&outward_normal
        };

        return Some(hittable::HitRecord {
            t: root,
            point: point,
            normal: normal,
            front_face,
        });
    }
}
