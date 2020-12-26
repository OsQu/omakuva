const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    // Render

    // Header
    println!("P3");
    println!("{} {} 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
