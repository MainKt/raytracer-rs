mod triple;

mod color;
mod point;
mod vector;

pub mod ray;

pub use color::*;
pub use point::*;
pub use ray::*;
pub use vector::*;

fn hits_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin.components - center.components;

    let a = ray.direction.components.length_squared();
    let half_b = oc.dot(&ray.direction.components);
    let c = oc.length_squared() - radius.powi(2);

    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn color_ray_on_sphere(ray: &Ray) -> Color {
    let t = hits_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t).components - Vector::new(0.0, 0.0, -1.0).components).unit();
        return (0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0).components)
            .into();
    }

    let direction = ray.direction.components.unit();
    let a = 0.5 * (direction.y() + 1.0);
    ((1.0 - a) * Color::new(1.0, 1.0, 1.0).components + a * Color::new(0.5, 0.7, 1.0).components)
        .into()
}
