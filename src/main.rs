mod tracer;
mod ray;
mod camera;
mod scene;
mod image;
mod vec3;
mod light;
mod object;
mod material;
mod geometry;

use crate::{
    tracer::Tracer,
    scene::Scene,
    camera::Camera,
    vec3::Vec3,
    object::Object,
    material::{lambertian::Lambertian},
    geometry::{sphere::Sphere, plane::Plane},
    image::Color, light::point_source::PointSource,
};

fn main() {
    let mut scene = Scene::new();
    scene.add_object(
        Object::new(
            Box::new(
                Lambertian::new(Color::new(0.8, 0.2, 0.2)),
            ),
            Box::new(
                Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5),
            ),
        )
    );
    scene.add_object(
        Object::new(
            Box::new(
                Lambertian::new(Color::new(0.2, 0.2, 0.8)),
            ),
            Box::new(
                Sphere::new(Vec3::new(0.6, 0.6, -0.6), 0.2),
            ),
        )
    );
    scene.add_object(
        Object::new(
            Box::new(
                Lambertian::new(Color::new(0.1, 0.8, 0.1)),
            ),
            Box::new(
                Plane::new(Vec3::new(0.0, 1.0, 0.0), -1.0),
            ),
        )
    );
    scene.add_light(
        Box::new(
            PointSource::new(
                Vec3::new(2.0, 2.0, -2.0),
                Color::new(1.0, 1.0, 1.0),
                5.0,
            ),
        )
    );
    scene.add_light(
        Box::new(
            PointSource::new(
                Vec3::new(-10.0, 10.0, 5.0),
                Color::new(1.0, 1.0, 1.0),
                50.0,
            ),
        )
    );
    scene.add_light(
        Box::new(
            PointSource::new(
                Vec3::new(0.0, 0.0, -3.0),
                Color::new(1.0, 1.0, 1.0),
                1.0,
            ),
        )
    );
    
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        80.0,
        1.0,
    );

    let tracer = Tracer::new(
        scene,
        camera,
        1000,
        1000,
        10,
        10,
    );
    let image = tracer.render();
    image.to_image().save("output.png").unwrap();
}

