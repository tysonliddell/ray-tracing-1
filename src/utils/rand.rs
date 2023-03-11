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

    // /// Get a random number in the provided range
    // pub fn random_range(&self, range: Range<f64>) -> f64 {
    //     range.start + (range.end - range.start) * self.random_f64()
    // }
}
