//! This module contains the [`Vec3`] type.
use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
/// Three-dimensional Euclidean vector.
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

const VEC3_DIM: usize = 3;

impl Vec3 {
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
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

    /// Get the dot product with another vector.
    pub fn dot(&self, other: Self) -> f64 {
        (0..VEC3_DIM).map(|i| self[i] * other[i]).sum()
    }

    /// Get the cross product with another vector.
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Get the unit vector in the direction given by the vector.
    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    // /// Normalize the current vector to have unit length.
    // pub fn normalize(&mut self) {
    //     *self /= self.length();
    // }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    /// Negate a vector.
    ///
    /// ```
    /// # use ray_tracing_1::geometry::vec3::Vec3;
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

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
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
mod tests {
    use super::Vec3;

    // A pythagorean quadruple (a,b,c,d) satisfies a*a + b*b + c*c == d*d
    const PYTHAGOREAN_QUADRUPLE: (f64, f64, f64, f64) = (4.0, 13.0, 16.0, 21.0);

    #[test]
    fn default() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), Vec3::default())
    }

    #[test]
    fn length() {
        let (x, y, z, d) = PYTHAGOREAN_QUADRUPLE;
        let v = Vec3::new(x, y, z);
        let d_sqrd = d * d;
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

        assert_eq!(Vec3::new(5.0, 7.0, 9.0), v1 + v2);

        let mut v1 = v1;
        v1 += v2;
        assert_eq!(Vec3::new(5.0, 7.0, 9.0), v1);
    }

    #[test]
    fn subtract() {
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(Vec3::new(-1.0, 0.0, 1.0), v1 - v2);

        let mut v1 = v1;
        v1 -= v2;
        assert_eq!(Vec3::new(-1.0, 0.0, 1.0), v1);
    }

    #[test]
    fn mul_pointwise() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(Vec3::new(2.0, 6.0, 12.0), v1 * v2);

        let mut v1 = v1;
        v1 *= v2;
        assert_eq!(Vec3::new(2.0, 6.0, 12.0), v1);
    }

    #[test]
    fn mul_with_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), v * 3.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), 3.0 * v);

        let mut v = v;
        v *= 2.0;
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), v);
    }

    #[test]
    fn div_with_scalar() {
        let v = Vec3::new(6.0, 12.0, 18.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), v / 2.0);

        let mut v = v;
        v /= 3.0;
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), v);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(2.0 + 6.0 + 12.0, v1.dot(v2));
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(Vec3::new(-1.0, 2.0, -1.0), v1.cross(&v2));
        assert_eq!(Vec3::new(1.0, -2.0, 1.0), v2.cross(&v1));
    }

    #[test]
    fn unit_vector() {
        let (x, y, z, d) = PYTHAGOREAN_QUADRUPLE;
        let v = Vec3::new(x, y, z);
        assert_eq!(Vec3::new(x / d, y / d, z / d), v.normalized());

        // let mut v = v;
        // v.normalize();
        // assert_eq!(Vec3::new(x / d, y / d, z / d), v);
    }

    #[test]
    fn index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!((1.0, 2.0, 3.0), (v[0], v[1], v[2]));

        let mut v = v;
        for i in 0..3 {
            v[i] *= 2.0;
        }
        assert_eq!((2.0, 4.0, 6.0), (v[0], v[1], v[2]));
    }
}
