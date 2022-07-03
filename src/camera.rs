use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub focal_length: f64,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let lower_left_corner = origin - (u * half_width) - (v * half_height) - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Camera {
            origin,
            lower_left_corner,
            focal_length: (look_from - look_at).length(),
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}

#[cfg(test)]
mod tests {
use crate::{
    Camera,
    vec3::Vec3,
};
use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_camera_get_ray() {
        let camera = Camera::new(
            Vec3::new(-4.0, 4.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            160.0,
            800.0 / 600.0,
        );
        let ray = camera.get_ray(0.5, 0.5);
        assert_eq!(
            ray.origin,
            Vec3::new(-4.0, 4.0, 1.0)
        );
        assert_approx_eq!(ray.direction.x, 2.0/3.0);
        assert_approx_eq!(ray.direction.y, -2.0/3.0);
        assert_approx_eq!(ray.direction.z, -1.0/3.0);
    }
}