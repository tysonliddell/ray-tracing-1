use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let co = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(co);
        let c = co.length_squared() - self.radius * self.radius;
        let descrim = half_b * half_b - a * c;

        // solve the quadratic equation
        let sqrt_d = (descrim > 0.0).then_some(descrim.sqrt())?;
        let root = Some((-half_b - sqrt_d) / a)
            .filter(|t| (t_min..t_max).contains(t))
            .or_else(|| Some((-half_b + sqrt_d) / a))
            .filter(|t| (t_min..t_max).contains(t))?;

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;

        // FIXME: Do we have to calculate `set_face_normal` as a separate step?
        let mut hr = HitRecord::new(point, outward_normal, t);
        hr.set_face_normal(ray, outward_normal);
        Some(hr)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn hit() {
        todo!()
    }
}
