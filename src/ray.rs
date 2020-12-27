use crate::vec3::*;

pub struct Ray<'a> {
    pub orig: &'a Point3,
    pub dir: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + &(self.dir * t)
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

        assert_eq!(ray.at(2.0), Vec3(2.0, 2.0, 1.0))
    }
}
