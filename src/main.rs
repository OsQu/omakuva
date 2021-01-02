// Image dimensions
mod camera;
mod color;
mod dielectric;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::color::*;
use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;
use rand::prelude::*;
use std::f32;

// Image dimensions
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 10;
const MAX_DEPTH: i32 = 50;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let world_hit = world.hit(&ray, 0.001, std::f32::INFINITY);
    match world_hit {
        Some(record) => {
            let scatter = record.material.scatter(&ray, &record);
            match scatter {
                Some((attenuation, scattered)) => {
                    return attenuation * ray_color(&scattered, world, depth - 1)
                }
                None => {
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
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
    let mut rng = rand::thread_rng();
    let camera = camera::Camera::new(90_f32, ASPECT_RATIO);

    // World

    let R: f32 = (f32::consts::PI / 4_f32).cos();

    let material_left = lambertian::Lambertian::new(Color::new(0_f32, 0_f32, 1_f32));
    let material_right = lambertian::Lambertian::new(Color::new(1_f32, 0_f32, 0_f32));

    let mut world = hittable_list::HittableList::new();

    world.add(Box::new(Sphere {
        center: Point3::new(-R, 0.0, -1.0),
        radius: R,
        material: &material_left,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(R, 0.0, -1.0),
        radius: R,
        material: &material_right,
    }));
    // Render
    eprintln!("Starting render");

    // Header
    println!("P3");
    println!("{} {} 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let color = (0..SAMPLES_PER_PIXEL).fold(Color::new(0.0, 0.0, 0.0), |acc, _sample| {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;

                return acc + ray_color(&camera.get_ray(u, v), &world, MAX_DEPTH);
            });

            color.write(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done.");
}
