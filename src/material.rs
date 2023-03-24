use std::fmt::Debug;

use crate::{
    color::Color,
    geometry::{hittable::HitRecord, ray::Ray},
    utils::rand::RTRng,
};

pub trait Material: Debug {
    /// Apply the properties of the material to attenuate the color of a reflected ray.
    fn attenuate(&self, color: Color) -> Color;

    /// Compute how a ray reflects off the material. Return `None` when the ray is absorbed
    /// rather than reflected.
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &RTRng) -> Option<Ray>;
}

#[derive(Debug)]
pub struct Lambertian {
    /// The reflection coefficient broken by color (red, green, blue).
    albedo: (f64, f64, f64),
}

impl Lambertian {
    pub fn new(albedo_red: f64, albedo_green: f64, albedo_blue: f64) -> Self {
        Self {
            albedo: (albedo_red, albedo_green, albedo_blue),
        }
    }
}

impl Material for Lambertian {
    fn attenuate(&self, color: Color) -> Color {
        let (red_att, green_att, blue_att) = self.albedo;
        Color {
            red: (color.red as f64 * red_att) as u8,
            green: (color.green as f64 * green_att) as u8,
            blue: (color.blue as f64 * blue_att) as u8,
        }
    }

    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, rng: &RTRng) -> Option<Ray> {
        let mut scatter_dir = hit_record.normal + rng.random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal;
        }

        // Lambertian diffusion always reflects the incident ray. `None` is never
        // returned.
        Some(Ray::new(hit_record.point, scatter_dir))
    }
}

#[derive(Debug)]
pub struct Metal {
    /// The reflection coefficient broken by colors (red, green, blue).
    albedo_rgb: (f64, f64, f64),

    /// Fuzziness factor for rays reflected off the metal. A value of `0.0` gives
    /// no perturbation (angle of incidence equal to angle or reflection), and larger
    /// values increase how far from the perfect reflection angle the reflected rays
    /// can deviate. Smoother surfaces have a lower fuzziness factor.
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo_rgb: (f64, f64, f64), fuzz: f64) -> Self {
        Self {
            albedo_rgb,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn attenuate(&self, color: Color) -> Color {
        let (red_att, green_att, blue_att) = self.albedo_rgb;
        Color {
            red: (color.red as f64 * red_att) as u8,
            green: (color.green as f64 * green_att) as u8,
            blue: (color.blue as f64 * blue_att) as u8,
        }
    }

    /// Metal simply scatters rays by reflection.
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &RTRng) -> Option<Ray> {
        let reflected = ray_in.direction().normalized().reflect(hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * rng.random_in_unit_sphere(),
        );

        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &RTRng) -> Option<Ray> {
        let refraction_ratio = if hit_record.front_face.unwrap() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let cos_theta = (-ray_in.direction().normalized().dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.random_f64()
        {
            ray_in.direction().reflect(hit_record.normal)
        } else {
            ray_in
                .direction()
                .refract(hit_record.normal, refraction_ratio)
        };

        Some(Ray::new(hit_record.point, direction))
    }

    fn attenuate(&self, color: Color) -> Color {
        color
    }
}
