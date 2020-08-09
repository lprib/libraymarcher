use crate::vec3::Vec3;

pub trait Color: Default + Copy + Clone {
    fn mul(self, scalar: f64) -> Self;
    fn add(self, other: Self) -> Self;
    fn white() -> Self;
}

impl Color for Vec3 {
    fn mul(self, scalar: f64) -> Self {
        scalar * self
    }

    fn add(self, other: Self) -> Self {
        self + other
    }
    fn white() -> Self {
        (1, 1, 1).into()
    }
}

impl Color for f64 {
    fn mul(self, scalar: f64) -> Self {
        self * scalar
    }

    fn add(self, other: Self) -> Self {
        self + other
    }
    fn white() -> Self {
        1.0
    }
}
