use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Triple(f64, f64, f64);

impl Triple {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl Default for Triple {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl ops::Neg for Triple {
    type Output = Triple;

    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add<Triple> for Triple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign<Triple> for Triple {
    fn add_assign(&mut self, rhs: Triple) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

impl ops::Sub<Triple> for Triple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Mul<Triple> for Triple {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::MulAssign<Triple> for Triple {
    fn mul_assign(&mut self, rhs: Triple) {
        self.0 *= rhs.x();
        self.1 *= rhs.y();
        self.2 *= rhs.z();
    }
}

impl ops::Mul<f64> for Triple {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }
}

impl ops::MulAssign<f64> for Triple {
    fn mul_assign(&mut self, scalar: f64) {
        self.0 *= scalar;
        self.1 *= scalar;
        self.2 *= scalar;
    }
}

impl ops::Div<f64> for Triple {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self(self.x() / scalar, self.y() / scalar, self.z() / scalar)
    }
}

impl ops::DivAssign<f64> for Triple {
    fn div_assign(&mut self, scalar: f64) {
        self.0 /= scalar;
        self.1 /= scalar;
        self.2 /= scalar;
    }
}

impl ops::Mul<Triple> for f64 {
    type Output = Triple;

    fn mul(self, rhs: Triple) -> Self::Output {
        rhs * self
    }
}
