//! A *Hittable* refers to something that can be *hit* by a ray in the scene.
//! This could be something concrete like a [`super::sphere::Sphere`], or
//! something more general, such as an array of surfaces.
use super::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    /// The point where a ray hits the hittable
    pub point: Vec3,

    /// The surface normal of the hittable at that point
    pub normal: Vec3,

    /// The [`Ray`] parameter for which this hit occurs.
    /// Corresponds to `point == Ray::at(t)`
    pub t: f64,
}

/// Used to determine whether a hittable in the scene has been hit by a ray.
pub trait Hittable {
    /// Returns a [`HitRecord`] describing where a given [`Ray`] hits the
    /// hittable or `None` if the ray does not hit it.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
