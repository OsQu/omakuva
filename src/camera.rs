use crate::ray;
use crate::vec3::*;

const FOCAL_LENGTH: f32 = 1.0;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    pub fn new(vertical_fov_deg: f32, aspect_ratio: f32) -> Camera {
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2_f32).tan();
        let viewport_height = 2_f32 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::new(0_f32, 0_f32, 0_f32);
        let horizontal = Vec3::new(viewport_width, 0_f32, 0_f32);
        let vertical = Vec3::new(0_f32, viewport_height, 0_f32);
        let lower_left_corner = &origin
            - &(&horizontal / 2_f32)
            - &vertical / 2_f32
            - Vec3::new(0_f32, 0_f32, FOCAL_LENGTH);

        return Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let dir = &(&self.lower_left_corner + &(u * &self.horizontal) + (v * &self.vertical))
            - &self.origin;

        return ray::Ray {
            orig: &self.origin,
            dir: dir,
        };
    }
}
