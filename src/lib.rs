mod utility;

use crate::utility::Color;
use indicatif::ProgressIterator;
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

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            pixel_color.render(&mut image)?;
        }
    }
    Ok(())
}
