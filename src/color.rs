use crate::vec3::Vec3;
use std::ops::{Add, Mul};

pub trait Color: Default + Copy + Clone
where
    Self: Mul<f64, Output = Self>,
    Self: Add<Self, Output = Self>,
{
    fn white() -> Self;
}

impl Color for Vec3 {
    fn white() -> Self {
        (1, 1, 1).into()
    }
}

impl Color for f64 {
    fn white() -> Self {
        1.0
    }
}
