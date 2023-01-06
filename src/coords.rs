#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord2d(f32, f32);
use bevy::prelude::*;

impl Coord2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn into_transform(self, z: f32) -> Transform {
        Transform::from_translation(Vec3::new(self.0, self.1, z))
    }
}

impl Into<(f32, f32)> for Coord2d {
    fn into(self) -> (f32, f32) {
        (self.0, self.1)
    }
}

use std::ops;

impl ops::Add for Coord2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Coord2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<f32> for Coord2d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Div<f32> for Coord2d {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}