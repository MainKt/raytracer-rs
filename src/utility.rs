mod triple;

mod color;
mod point;
mod vector;

pub mod ray;

pub use color::*;
pub use point::*;
pub use ray::*;
pub use vector::*;

fn hits_sphere(center: &Point, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin.components - center.components;

    let a = ray.direction.components.dot(&ray.direction.components);
    let b = 2.0 * oc.dot(&ray.direction.components);
    let c = oc.dot(&oc) - radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;
    discriminant >= 0.0
}

pub fn color_ray_on_sphere(ray: &Ray) -> Color {
    if hits_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let direction = ray.direction.components.unit();
    let a = 0.5 * (direction.y() + 1.0);
    ((1.0 - a) * Color::new(1.0, 1.0, 1.0).components + a * Color::new(0.5, 0.7, 1.0).components)
        .into()
}
