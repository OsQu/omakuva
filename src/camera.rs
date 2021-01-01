use crate::ray;
use crate::vec3::*;

// Image dimensions
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

// Viewport
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    pub fn new() -> Camera {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

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
