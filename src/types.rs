use bevy::prelude::*;
use rand::prelude::*;

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
    pub objtype: Objecttype,
}

impl Object {
    pub fn dummy() -> Self {
        Self {
            spawn_time: 0.,
            objtype: Objecttype::Normal,
        }
    }

    pub fn new(spawn_time: f64, objtype: Objecttype) -> Self {
        Self {
            spawn_time,
            objtype,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Fumen {
    pub objects: Vec<Object>,
}

impl Fumen {
    pub fn dummy() -> Self {
        let mut objects = vec![];
        for i in 0..10 {
            for _ in 0..5 {
                let time: f64 = rand::distributions::Uniform::new_inclusive(0.0, 1.0).sample(&mut rand::thread_rng());
                objects.push(Object::new(i as f64 + time, Objecttype::Normal));
            }
        }
        Self {
            objects,
        }
    }
}