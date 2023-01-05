use bevy::prelude::*;
use rand::prelude::*;
use crate::consts::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Objecttype {
    Normal,
    Vertical,
    Top(u32),
}

#[derive(Component, Clone, Copy, Debug)]
/// Keeps track of when each arrow should spawn and it's speed and direction
pub struct Object {
    pub spawn_time: f64,
    pub spawn_x: f32,
    pub objtype: Objecttype,
}

impl Object {
    pub fn dummy() -> Self {
        Self {
            spawn_time: 0.,
            spawn_x: 0.,
            objtype: Objecttype::Normal,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Fumen {
    pub objects: Vec<Object>,
}

use crate::utils::range_rng;
impl Fumen {
    pub fn dummy() -> Self {
        let mut objects = vec![];
        for i in 0..10 {
            for _ in 0..10 {
                let time: f64 = range_rng(0., 1.) + i as f64;
                let spawn_x: f32 = range_rng(INNER_WINDOW_X_MIN, INNER_WINDOW_X_MAX);
                objects.push(Object{
                    spawn_time: time,
                    spawn_x,
                    objtype: Objecttype::Normal,
                });
            }
        }
        objects.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());
        Self {
            objects,
        }
    }
}