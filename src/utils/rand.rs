use std::ops::Range;

use crate::geometry::vec3::Vec3;

/// Random number generator for the ray tracer.
pub struct RTRng {
    rng: fastrand::Rng,
}

impl RTRng {
    pub fn new() -> Self {
        Self {
            rng: fastrand::Rng::new(),
        }
    }

    /// Get a random number in the range [0,1)
    pub fn random_f64(&self) -> f64 {
        self.rng.f64()
    }

    /// Get a random number in the provided range
    pub fn random_f64_range(&self, range: Range<f64>) -> f64 {
        range.start + (range.end - range.start) * self.random_f64()
    }

    // fn random_vec3(&self) -> Vec3 {
    //     (self.random_f64(), self.random_f64(), self.random_f64()).into()
    // }

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

    pub fn random_unit_vector(&self) -> Vec3 {
        self.random_in_unit_sphere().normalized()
    }
}
