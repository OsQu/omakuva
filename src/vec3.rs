use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
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
    fn test_neg() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let negated_vector = -vector;

        assert_eq!(negated_vector, Vec3(-1.0, -2.0, -3.0))
    }

    #[test]
    fn test_add_assign() {
        let mut vector = Vec3(1.0, 2.0, 3.0);
        vector += Vec3(1.0, 2.0, 3.0);

        assert_eq!(vector, Vec3(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_mul_assign_f32() {
        let mut vector = Vec3(1.0, 2.0, 3.0);
        vector *= 3.0;

        assert_eq!(vector, Vec3(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_div_assign() {
        let mut vector = Vec3(2.0, 4.0, 6.0);
        vector /= 2.0;

        assert_eq!(vector, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_length() {
        let vector = Vec3(2.0, 4.0, 4.0);

        assert_eq!(vector.length(), 6.0)
    }
}
