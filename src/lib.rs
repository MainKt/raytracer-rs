mod utility;

use crate::utility::{Point, Ray, Vector};
use indicatif::ProgressIterator;
use std::fs::File;
use std::io::{self, Write};

pub fn run(path: String) -> io::Result<()> {
    let image = File::create(path)?;

    render(image)
}

fn render(mut image: File) -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f64;
    let camera_center = Point::default(); // (0, 0, 0)

    let viewport_u = Vector::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.components / image_width as f64;
    let pixel_delta_v = viewport_v.components / image_height as f64;

    let viewport_upper_left = camera_center.components
        - Vector::new(0.0, 0.0, focal_length).components
        - viewport_u.components / 2.0
        - viewport_v.components / 2.0;
    let viewport_upper_left: Vector = viewport_upper_left.into();
    let pixel00_location = viewport_upper_left.components + 0.5 * (pixel_delta_u + pixel_delta_v);

    writeln!(image, "P3\n{image_width} {image_height}\n255")?;

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_center =
                pixel00_location + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center.components;

            let ray = Ray {
                origin: camera_center,
                direction: ray_direction.into(),
            };

            let pixel_color = ray.color();

            pixel_color.render(&mut image)?;
        }
    }
    Ok(())
}
