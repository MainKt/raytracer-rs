use super::triple::Triple;

#[derive(Default)]
pub struct Vector {
    pub components: Triple,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: Triple::new(x, y, z),
        }
    }
}

impl From<Triple> for Vector {
    fn from(components: Triple) -> Self {
        Self { components }
    }
}
