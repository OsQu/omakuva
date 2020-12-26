// Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

// Viewport
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

mod color;
mod ray;
mod vec3;

use crate::vec3::*;

fn main() {
    // Camera
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    eprintln!("Starting render");

    // Header
    println!("P3");
    println!("{} {} 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            color::Color::new(r, g, b).write();
        }
    }

    let foo = vec3::Vec3(1.0, 2.0, 3.0);
    foo.length();
    eprintln!("Done.");
}
