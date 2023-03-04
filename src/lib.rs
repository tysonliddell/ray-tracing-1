//! Basic ray-tracing library.
use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
/// Three-dimensional Euclidean vector.
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Get the Euclidean length of a vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Get the Euclidean length of a vector squared.
    pub fn length_squared(&self) -> f64 {
        let Self { x, y, z } = self;
        x * x + y * y + z * z
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    /// Negate a vector.
    ///
    /// ```
    /// # use ray_tracing_1::Vec3;
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let neg_v = Vec3::new(-1.0, -2.0, -3.0);
    /// assert_eq!(neg_v, -v);
    /// ```
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    /// Access a component of the vector by index.
    ///
    /// # Panics
    /// Will panic if attempting to access an index other than 0, 1 or 2.
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of range."),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of range."),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Vec3;

    #[test]
    fn default() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), Vec3::default())
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let d_sqrd = 1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0;
        assert_eq!(d_sqrd, v.length_squared());
        assert_eq!(d_sqrd.sqrt(), v.length());
    }

    #[test]
    fn neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let mut v3 = v1 + v2;

        assert_eq!(Vec3::new(5.0, 7.0, 9.0), v3);

        v3 += v1;
        assert_eq!(Vec3::new(6.0, 9.0, 12.0), v3);
    }

    #[test]
    fn mul() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), v1 * 3.0);

        v1 *= 2.0;
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), v1);
    }

    #[test]
    fn div() {
        let mut v1 = Vec3::new(6.0, 12.0, 18.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), v1 / 2.0);

        v1 /= 3.0;
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), v1);
    }

    #[test]
    fn index() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!((1.0, 2.0, 3.0), (v[0], v[1], v[2]));

        for i in 0..3 {
            v[i] *= 2.0;
        }
        assert_eq!((2.0, 4.0, 6.0), (v[0], v[1], v[2]));
    }
}
