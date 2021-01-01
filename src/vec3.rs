use rand::prelude::*;
use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3(x, y, z);
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        return Vec3(rng.gen(), rng.gen(), rng.gen());
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::random_in_unit_sphere().unit_vector();
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random();
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        other * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        (1.0 / t) * self
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        (1.0 / t) * self
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_in_unit_sphere() {
        let p = Vec3::random_in_unit_sphere();
        assert_eq!(p.length_squared() < 1.0, true);
    }
    #[test]
    fn test_neg() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let negated_vector = -&vector;

        assert_eq!(negated_vector, Vec3(-1.0, -2.0, -3.0))
    }

    #[test]
    fn test_add() {
        assert_eq!(
            &Vec3(1.0, 1.0, 1.0) + &Vec3(2.0, 2.0, 2.0),
            Vec3(3.0, 3.0, 3.0)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut vector = Vec3(1.0, 2.0, 3.0);
        vector += Vec3(1.0, 2.0, 3.0);

        assert_eq!(vector, Vec3(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            &Vec3(3.0, 2.0, 1.0) - &Vec3(1.0, 1.0, 1.0),
            Vec3(2.0, 1.0, 0.0)
        )
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            &Vec3(1.0, 2.0, 3.0) * &Vec3(1.0, 2.0, 3.0),
            Vec3(1.0, 4.0, 9.0)
        )
    }

    #[test]
    fn test_mul_f32() {
        assert_eq!(&Vec3(1.0, 2.0, 3.0) * 2.0, Vec3(2.0, 4.0, 6.0));
        assert_eq!(2.0 * &Vec3(1.0, 2.0, 3.0), Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign_f32() {
        let mut vector = Vec3(1.0, 2.0, 3.0);
        vector *= 3.0;

        assert_eq!(vector, Vec3(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_div() {
        assert_eq!(&Vec3(9.0, 6.0, 3.0) / 3.0, Vec3(3.0, 2.0, 1.0));
    }

    #[test]
    fn test_div_assign() {
        let mut vector = Vec3(2.0, 4.0, 6.0);
        vector /= 2.0;

        assert_eq!(vector, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_dot() {
        let vector = Vec3(1.0, 2.0, 3.0);
        assert_eq!(vector.dot(&Vec3(1.0, 5.0, 7.0)), 32.0);
    }

    #[test]
    fn test_cross() {
        let vector = Vec3(1.0, 2.0, 3.0);
        assert_eq!(vector.cross(&Vec3(1.0, 5.0, 7.0)), Vec3(-1.0, -4.0, 3.0))
    }

    #[test]
    fn test_unit_vector() {
        let vector = Vec3(2.0, 4.0, 4.0);
        assert_eq!(vector.unit_vector(), Vec3(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0))
    }

    #[test]
    fn test_length() {
        let vector = Vec3(2.0, 4.0, 4.0);

        assert_eq!(vector.length(), 6.0)
    }
}
