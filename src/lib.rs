use std::fs::File;
use std::io::{self, Write};

pub fn run(path: String) -> io::Result<()> {
    let image = File::create(path)?;

    render(image)
}

fn render(mut image: File) -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    writeln!(image, "P3\n{image_width} {image_height}\n255")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let r = (255.999 * r) as i32;
            let g = (255.999 * g) as i32;
            let b = (255.999 * b) as i32;

            writeln!(image, "{r} {g} {b}")?;
        }
    }

    Ok(())
}
