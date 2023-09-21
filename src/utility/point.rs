use super::triple::Triple;

#[derive(Default, Clone, Copy)]
pub struct Point {
    pub components: Triple,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: Triple::new(x, y, z),
        }
    }
}

impl From<Triple> for Point {
    fn from(components: Triple) -> Self {
        Self { components }
    }
}
