use crate::utils;
use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }
    pub fn write(&self, samples_per_pixel: i32) {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        let scale = 1.0 / (samples_per_pixel as f32);

        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir = (256.0 * utils::clamp(r, 0.0, 0.999)) as u32;
        let ig = (256.0 * utils::clamp(g, 0.0, 0.999)) as u32;
        let ib = (256.0 * utils::clamp(b, 0.0, 0.999)) as u32;
        println!("{} {} {}", ir, ig, ib)
    }
}
