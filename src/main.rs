const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

mod color;
mod ray;
mod vec3;

fn main() {
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
