use super::triple::Triple;
use std::io;
use std::io::Write;

#[derive(Default)]
pub struct Color {
    components: Triple,
}

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: Triple::new(x, y, z),
        }
    }

    pub fn components(&self) -> Triple {
        self.components
    }

    pub fn r(&self) -> i32 {
        (255.999 * self.components().x()) as i32
    }
    pub fn g(&self) -> i32 {
        (255.999 * self.components().y()) as i32
    }
    pub fn b(&self) -> i32 {
        (255.999 * self.components().z()) as i32
    }

    pub fn render(&self, image: &mut impl Write) -> io::Result<()> {
        writeln!(image, "{} {} {}", self.r(), self.g(), self.b())
    }
}
