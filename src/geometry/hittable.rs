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

    /// Indicates whether or not the hit occurred at point that
    /// faces the origin of the incident `Ray`. When set to `None`
    /// the value hasn't been calculated yet.
    pub front_face: Option<bool>,
    // TODO:
    // - Determine if `normal` needs to be an `Option<Vec3>`.
    // - Given that `t` relates to an incident `Ray`, it looks like
    //   all data here should be populated when the ray hits. I don't
    //   see the need for evaluating `set_face_normal` later.
    //   Change this if it makes sense to.
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face: None,
        }
    }

    /// Set the `front_face` and `normal` of the `HitRecord` such that:
    /// - `front_face` is true if and only if the hit occurred at a point
    ///   that faces the source of the incident [`Ray`].
    /// - The `normal` points against the incident ray.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let is_front_face = ray.direction().dot(outward_normal) < 0.0;

        self.front_face = Some(is_front_face);
        self.normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// Used to determine whether a hittable in the scene has been hit by a ray.
pub trait Hittable {
    /// Returns a [`HitRecord`] describing where a given [`Ray`] hits the
    /// hittable or `None` if the ray does not hit it.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T> Hittable for &[T]
where
    T: AsRef<dyn Hittable>,
{
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        let mut closest = None;
        for hittable in self.iter().map(|x| x.as_ref()) {
            if let Some(hr) = hittable.hit(ray, t_min, t_max) {
                t_max = hr.t;
                closest = Some(hr);
            }
        }

        closest
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn hit_on_vec_of_hittable() {
        todo!(
            "Write this test later when we have more than one type of \
            hittable to test in a single slice."
        )
    }
}
