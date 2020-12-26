use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
    pub fn write(&self) {
        println!("{} {} {}", self.0, self.1, self.2)
    }
}
