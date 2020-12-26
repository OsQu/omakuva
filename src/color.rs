use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        return vec3::Vec3(r, g, b);
    }

    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }
    pub fn write(&self) {
        let ir = (255.999 * self.r()) as u32;
        let ig = (255.999 * self.g()) as u32;
        let ib = (255.999 * self.b()) as u32;
        println!("{} {} {}", ir, ig, ib)
    }
}
