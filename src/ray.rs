use crate::vec3::*;

pub struct Ray<'a> {
    orig: &'a Point3,
    dir: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + &(&self.dir.unit_vector() * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray {
            orig: &Vec3(0.0, 0.0, 1.0),
            dir: &Vec3(1.0, 1.0, 0.0),
        };

        assert_eq!(ray.at(2.0), Vec3(2.0_f32.sqrt(), 2.0_f32.sqrt(), 1.0))
    }
}
