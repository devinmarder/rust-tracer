use crate::{
    material,
    material::Material,
    geometry,
    geometry::{Geometry, Hit},
    ray::Ray,
    light::Light,
    image::Color,
};

pub struct Object{
    pub material: Box<dyn Material>,
    pub geometry: Box<dyn Geometry>,
}

impl Object {
    pub fn new(material: Box<dyn Material>, geometry: Box<dyn Geometry>) -> Object {
        Object {
            material,
            geometry,
        }
    }
    pub fn default() -> Object {
        Object {
            material: material::default(),
            geometry: geometry::default(),
        }
    }
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.geometry.hit(ray)
    }
    pub fn shade(&self, ray: &Ray, hit: &Hit, lights: &Vec<Box<dyn Light>>, objects: &Vec<Object>, depth: u32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in lights {
            let light_sample = light.sample();
            let light_ray = Ray::new(light_sample, (hit.point - light_sample).normalize());
            if self.is_shadow(&hit, &light_ray, objects) {
                continue;
            } else {
                self.material.shade(&light_ray, &hit, &light, depth + 1, &mut color);
            }
        }
        color
    }
    fn is_shadow(&self, hit: &Hit, light_ray: &Ray, objects: &Vec<Object>) -> bool {
        if hit.normal.dot(&light_ray.direction) > 0.0 {
            return true;
        }
        for other in objects {
            if self as *const _ != other as *const _ {
                if let Some(shadow_hit) = other.hit(light_ray) {
                    if shadow_hit.distance < (hit.point - light_ray.origin).length() {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        vec3::Vec3,
        ray::Ray,
        geometry::sphere::Sphere,
    };

    #[test]
    fn test_is_shadow_true() {
        let hit = Hit{
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            distance: 0.0,
        };
        let light_ray = Ray::new(Vec3::new(4.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        let object = Object::new(
            material::default(),
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0)),
        );
        let other = Object::new(
            material::default(),
            Box::new(Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0)),
        );
        let objects = vec![
            object,
            other,
        ];
        assert_eq!(true, objects[0].is_shadow(&hit, &light_ray, &objects));
    }
    #[test]
    fn test_is_shadow_false() {
        let hit = Hit{
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            distance: 0.0,
        };
        let light_ray = Ray::new(Vec3::new(4.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        let object = Object::new(
            material::default(),
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0)),
        );
        let other = Object::new(
            material::default(),
            Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 1.0)),
        );
        let objects = vec![
            object,
            other,
        ];
        assert_eq!(false, objects[0].is_shadow(&hit, &light_ray, &objects));
    }

}