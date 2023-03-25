use crate::geometry::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

/// Configuration used when constructing a [`Camera`].
pub struct Config {
    /// The point where the camera is located.
    pub look_from: Vec3,

    /// The point the camera is looking at.
    pub look_at: Vec3,

    /// The 'up' direction of the camera. Note that the vector from `look_from` to `look_at`
    /// is already normal to a plane, and the `vup` vector is projected onto this plane
    /// to orientate the camera about this `look_at - look_from` axis.
    pub vup: Vec3,

    /// Vertical field of view in degrees.
    pub vfov_degrees: f64,

    /// The aspect ratio of the viewport. Used to determine the horizontal fov from `vfov`.
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(config: Config) -> Self {
        let theta = config.vfov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = config.aspect_ratio * viewport_height;

        let w = (config.look_from - config.look_at).normalized();
        let u = config.vup.cross(w).normalized();
        let v = w.cross(u);

        let origin = config.look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Camera {
    /// Get a ray from the origin of the camera, to a point `(s, t)` on its viewport.
    /// `s` and `t` are the horizontal and vertical proportions of the viewport,
    /// respectively, starting from its bottom left corner.
    ///
    /// # Examples
    /// Get rays at different viewport positions
    /// ```
    /// # use ray_tracing_1::camera::Camera;
    /// let camera = Camera::new(16.0 / 9.0);
    /// let ray_at_viewport_bottom_left = camera.get_ray(0.0, 0.0);
    /// let ray_at_viewport_center = camera.get_ray(0.5, 0.5);
    /// let ray_at_viewport_top_right = camera.get_ray(1.0, 1.0);
    /// let ray_outside_viewport = camera.get_ray(1.1, 1.0);
    /// ```
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let dir = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;
        Ray::new(self.origin, dir)
    }
}
