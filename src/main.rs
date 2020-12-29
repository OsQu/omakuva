// Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

// Viewport
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::color::*;
use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let world_hit = world.hit(&ray, 0.0, std::f32::INFINITY);

    match world_hit {
        Some(record) => {
            return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
        }
        None => {
            // No hit, render background
            let unit_direction = ray.dir.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);

            return &((1.0 - t) * &Color::new(1.0, 1.0, 1.0)) + &(t * &Color::new(0.5, 0.7, 1.0));
        }
    }
}

fn main() {
    // Camera
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // Render
    eprintln!("Starting render");

    // Header
    println!("P3");
    println!("{} {} 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let dir = &(&lower_left_corner + &(u * &horizontal) + (v * &vertical)) - &origin;
            let ray = Ray {
                orig: &origin,
                dir: &dir,
            };

            let color = ray_color(&ray, &world);
            color.write();
        }
    }

    let foo = vec3::Vec3(1.0, 2.0, 3.0);
    foo.length();
    eprintln!("Done.");
}
