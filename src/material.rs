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
    albedo: (f64, f64, f64),
}

impl Metal {
    pub fn new(albedo_red: f64, albedo_green: f64, albedo_blue: f64) -> Self {
        Self {
            albedo: (albedo_red, albedo_green, albedo_blue),
        }
    }
}

impl Material for Metal {
    fn attenuate(&self, color: Color) -> Color {
        let (red_att, green_att, blue_att) = self.albedo;
        Color {
            red: (color.red as f64 * red_att) as u8,
            green: (color.green as f64 * green_att) as u8,
            blue: (color.blue as f64 * blue_att) as u8,
        }
    }

    /// Metal simply scatters rays by reflection.
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, _rng: &RTRng) -> Option<Ray> {
        let reflected = ray_in.direction().normalized().reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);

        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}
