use std::ops::Range;

use crate::geometry::vec3::Vec3;

/// Random number generator for the ray tracer.
pub struct RTRng {
    rng: fastrand::Rng,
}

impl RTRng {
    pub fn new() -> Self {
        let rng = fastrand::Rng::new();
        rng.seed(0);

        Self { rng }
    }

    /// Get a random number in the range [0,1)
    pub fn random_f64(&self) -> f64 {
        self.rng.f64()
    }

    /// Get a random number in the provided range
    pub fn random_f64_range(&self, range: Range<f64>) -> f64 {
        range.start + (range.end - range.start) * self.random_f64()
    }

    pub fn random_vec3(&self) -> Vec3 {
        (self.random_f64(), self.random_f64(), self.random_f64()).into()
    }

    pub fn random_vec3_range(&self, range: Range<f64>) -> Vec3 {
        (
            self.random_f64_range(range.clone()),
            self.random_f64_range(range.clone()),
            self.random_f64_range(range),
        )
            .into()
    }

    pub fn random_in_unit_sphere(&self) -> Vec3 {
        loop {
            let p = Vec3::from((
                self.random_f64_range(-1.0..1.0),
                self.random_f64_range(-1.0..1.0),
                self.random_f64_range(-1.0..1.0),
            ));

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Get a random vector on the unit sphere. Used for Lambertian reflection/diffusion.
    pub fn random_unit_vector(&self) -> Vec3 {
        self.random_in_unit_sphere().normalized()
    }

    /// Get a random vector on the unit sphere in the same hemisphere as the unit normal.
    /// An alternative, uniform diffuse formulation.
    #[allow(unused)]
    pub fn random_in_hemisphere(&self, normal: Vec3) -> Vec3 {
        let in_unit_sphere = self.random_unit_vector();
        if normal.dot(in_unit_sphere) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Get a random vector in a the unit disk in the `z=0` plane.
    pub fn random_in_unit_disk(&self) -> Vec3 {
        loop {
            let p = Vec3::new(
                self.random_f64_range(-1.0..1.0),
                self.random_f64_range(-1.0..1.0),
                0.0,
            );

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl Default for RTRng {
    fn default() -> Self {
        Self::new()
    }
}
