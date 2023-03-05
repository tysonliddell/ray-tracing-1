//! This module contains the [`Ray`] type.
use super::vec3::Vec3;

/// A ray with an origin, direction and the ability to compute any
/// position in space along it.
#[derive(Debug, Default, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// The point in space that the `Ray` starts from.
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// The direction the `Ray` is pointing from its origin.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Compute the position along the `Ray`, starting from its origin,
    /// at time `t`. Positive values of `t` yield positions in front
    /// of the origin and negative values yield positions behind the
    /// origin.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod test {
    use super::super::vec3::Vec3;
    use super::Ray;

    #[test]
    fn traverse() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, ray.at(0.0));
        assert_eq!(Vec3::new(0.0, -1.0, -2.0), ray.at(-1.0));
        assert_eq!(Vec3::new(2.0, 3.0, 4.0), ray.at(1.0));
        assert_eq!(Vec3::new(4.0, 7.0, 10.0), ray.at(3.0));
    }
}
