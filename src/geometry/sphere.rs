use std::{fmt::Debug, rc::Rc};

use crate::material::Material;

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new<T: Into<f64>>(center: Vec3, radius: T, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.into(),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let co = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(co);
        let c = co.length_squared() - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;

        // solve the quadratic equation
        let sqrt_d = (discrim >= 0.0).then_some(discrim.sqrt())?;
        let root = Some((-half_b - sqrt_d) / a)
            .filter(|&t| t > t_min && t < t_max)
            .or_else(|| Some((-half_b + sqrt_d) / a))
            .filter(|&t| t > t_min && t < t_max)?;

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;

        // FIXME: Do we have to calculate `set_face_normal` as a separate step?
        let mut hr = HitRecord::new(point, outward_normal, t, Rc::clone(&self.material));
        hr.set_face_normal(ray, outward_normal);
        Some(hr)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{hittable::Hittable, ray::Ray, vec3::Vec3};
    use super::Sphere;

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5);
        let miss_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert!(sphere.hit(&miss_ray, -10.0, 10.0).is_none());
    }

    #[test]
    fn ray_at_sphere_bullseye() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5);
        let bullseye_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        let hr = sphere
            .hit(&bullseye_ray, -1.0, 1.0)
            .expect("Ray should hit sphere");
        assert_eq!(0.5, hr.t);
        assert_eq!(Vec3::new(0.0, 0.0, 0.5), hr.point);
    }

    #[test]
    fn ray_t_bounds() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5);
        let bullseye_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(sphere.hit(&bullseye_ray, -1.0, 0.4999).is_none());
        assert!(sphere.hit(&bullseye_ray, -1.0, 0.5001).is_some());
    }

    #[test]
    fn ray_scrapes_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5);
        let scrape_dir = Vec3::new(0.0, 1.0, 3.0_f64.sqrt());

        // The scrape direction is where the ray is tangent to the sphere,
        // which won't play nicely with floating point approximations.
        // So we use a small delta direction that will move the ray closer
        // to, or further away from the sphere.
        let delta_dir = Vec3::new(0.0, 0.0, 0.0001);

        let scrape_ray_hit = Ray::new(Vec3::new(0.0, 0.0, 0.0), scrape_dir + delta_dir);
        let scrape_ray_miss = Ray::new(Vec3::new(0.0, 0.0, 0.0), scrape_dir - delta_dir);
        assert!(sphere.hit(&scrape_ray_hit, -1.0, 1.0).is_some());
        assert!(sphere.hit(&scrape_ray_miss, -1.0, 1.0).is_none());
    }
}
