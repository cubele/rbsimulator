#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord2d(f32, f32);
use bevy::prelude::*;

const EPS: f32 = 1e-8;

impl Coord2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn into_transform(self, z: f32) -> Transform {
        Transform::from_translation(Vec3::new(self.0, self.1, z))
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }

    pub fn slope(&self, other: &Self) -> Option<f32> {
        if (self.0 - other.0).abs() < EPS {
            None
        } else {
            Some((other.1 - self.1) / (other.0 - self.0))
        }
    }

    pub fn angle(&self, other: &Self) -> f32 {
        if let Some(angle) = self.slope(other) {
             let rad = angle.atan();
             if rad < 0. {
                rad + std::f32::consts::PI
            } else {
                rad
             }
        } else {
            std::f32::consts::FRAC_PI_2
        }
    }
}

impl Into<(f32, f32)> for Coord2d {
    fn into(self) -> (f32, f32) {
        (self.0, self.1)
    }
}

impl From<(f32, f32)> for Coord2d {
    fn from((x, y): (f32, f32)) -> Self {
        Self(x, y)
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