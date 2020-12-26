use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec3(f32, f32, f32);

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
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
}
