use crate::vec3::Vec3;
use core::ops::Mul;
use std::ops::Add;

trait Color {
    fn mul(self, other: f64) -> Self;
    fn add(self, other: Self) -> Self;
}

impl Color for Vec3 {
    fn mul(self, other: f64) -> Self {
        other * self
    }

    fn add(self, other: Self) -> Self {
        self + other
    }
}

impl Color for f64 {
    fn mul(self, other: f64) -> Self {
        self * other
    }

    fn add(self, other: Self) -> Self {
        self + other
    }
}