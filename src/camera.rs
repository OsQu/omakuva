use crate::ray;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_to: Point3,
        vup: Vec3,
        vertical_fov_deg: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2_f32).tan();
        let viewport_height = 2_f32 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = look_from.clone();

        // Camera direction
        let w = (look_from - look_to).unit_vector();
        // Camera plane
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = &origin - &(&horizontal / 2_f32) - &vertical / 2_f32 - w;

        return Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        };
    }

    pub fn get_ray(&self, s: f32, t: f32) -> ray::Ray {
        let dir = &(&self.lower_left_corner + &(s * &self.horizontal) + (t * &self.vertical))
            - &self.origin;

        return ray::Ray {
            orig: &self.origin,
            dir: dir,
        };
    }
}
