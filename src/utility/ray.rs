use super::Point;
use super::Vector;
use crate::utility::Color;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    // pub fn at(&self, t: f64) -> Point {
    //     (self.origin.components + t * self.direction.components).into()
    // }

    pub fn color(&self) -> Color {
        let direction = self.direction.components.unit();
        let a = 0.5 * (direction.y() + 1.0);

        ((1.0 - a) * Color::new(1.0, 1.0, 1.0).components
            + a * Color::new(0.5, 0.7, 1.0).components)
            .into()
    }
}
