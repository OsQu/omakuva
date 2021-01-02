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

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / camera::ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 100;
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
    let camera = camera::Camera::new();

    let material_ground = lambertian::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = lambertian::Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = dielectric::Dielectric::new(1.5);
    let material_right = metal::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let mut world = hittable_list::HittableList::new();

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &material_ground,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: &material_center,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: &material_left,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
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
