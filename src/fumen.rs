use bevy::prelude::*;
use crate::consts::*;
use crate::objects::{Object, Objecttype};

#[derive(Resource, Debug)]
pub struct Fumen {
    pub objects: Vec<Object>,
    pub current: usize,
    pub song_audio: Handle<AudioSource>,
    pub playing: bool,
}

use crate::utils::range_rng;
impl Fumen {
    pub fn dummy(asset_server: &AssetServer) -> Self {
        let song_audio = asset_server.load("..\\fumens\\INORI\\song.ogg");
        let mut objects = vec![];
        for i in 0..10 {
            for j in 0..5 {
                let spawn_time: f64 = 1.0 / j as f64 + i as f64;
                let arrive_time: f64 = spawn_time + 1.2;
                let pos: u32 = range_rng(0, 6);
                let spawn_x: f32 = range_rng(INNER_WINDOW_X_MIN, INNER_WINDOW_X_MAX);
                if let Some(object) = Object::new(
                    spawn_time, arrive_time, 
                    spawn_x, Objecttype::Normal, pos) {
                    objects.push(object);
                }
            }
            for j in 0..3 {
                let spawn_time: f64 = 1.0 / j as f64 + i as f64;
                let arrive_time: f64 = spawn_time + 1.2;
                let pos: u32 = range_rng(0, 2);
                let spawn_x: f32 = TOP_SPAWN_X_START + TOP_SPAWN_X_SPACING * pos as f32;
                if let Some(object) = Object::new(
                    spawn_time, arrive_time, 
                    spawn_x, Objecttype::Top, pos) {
                    objects.push(object);
                }
            }
        }
        objects.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());
        Self {
            objects,
            current: 0,
            song_audio,
            playing: false,
        }
    }

    pub fn current_object(&self) -> Option<&Object> {
        self.objects.get(self.current as usize)
    }
}