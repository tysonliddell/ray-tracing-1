use crate::geometry::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = (0, 0, 0).into();
        let horizontal = (viewport_width, 0, 0).into();
        let vertical = (0, viewport_height, 0).into();
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - (0, 0, focal_length).into();

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Camera {
    /// Get a ray from the origin of the camera, to a point `(u, v)` on its viewport.
    /// `u` and `v` are the horizontal and vertical proportions of the viewport,
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
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, dir)
    }
}
