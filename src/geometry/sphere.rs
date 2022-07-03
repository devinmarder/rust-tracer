use crate::{
    vec3::Vec3,
    ray::Ray,
    geometry::Hit,
};

use super::Geometry;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > 0.0 {
                Some(Hit {
                    distance: temp,
                    normal: (ray.origin + ray.direction * temp) - self.center,
                    point: ray.origin + ray.direction * temp,
                })
            } else {
                let temp = (-b + discriminant.sqrt()) / a;
                if temp > 0.0 {
                    Some(Hit {
                        distance: temp,
                        normal: (ray.origin + ray.direction * temp) - self.center,
                        point: ray.origin + ray.direction * temp,
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
    fn sample(&self, _: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    fn normal_at(&self, p: &Vec3) -> Vec3 {
        (p - &self.center).normalize()
    }
    fn hit_point(&self, ray: &Ray) -> Vec3 {
        ray.origin + ray.direction * self.radius
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    fn test_hit(geometry: &dyn Geometry, ray: &Ray, expected: Option<Hit>) {
        let actual = geometry.hit(ray);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 2.0));
        let expected = Some(Hit {
            distance: 4.0,
            normal: Vec3::new(0.0, 0.0, -1.0),
            point: Vec3::new(0.0, 0.0, -1.0),
        });
        test_hit(&sphere, &ray, expected);
    }
    #[test]
    fn test_sphere_miss() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0));
        let expected = None;
        test_hit(&sphere, &ray, expected);
    }
}