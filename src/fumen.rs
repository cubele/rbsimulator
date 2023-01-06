use bevy::prelude::*;
use crate::consts::*;
use crate::objects::{Object, Objecttype};

#[derive(Resource, Debug)]
pub struct Fumen {
    pub objects: Vec<Object>,
    pub current: usize,
    pub song_audio: Handle<AudioSource>,
    pub playing: bool,
    pub song_start_time: f64,
}

use crate::utils::range_rng;
impl Fumen {
    pub fn dummy(asset_server: &AssetServer) -> Self {
        let delay = 1.985;
        let song_audio = asset_server.load("..\\fumens\\INORI\\song.ogg");
        let bpm: f64 = 146.0;
        // quarter note
        let seconds_per_beat: f64 = 60.0 / bpm;
        let mut objects = vec![];
        let count = 2000;
        let mut last_pos = 10;
        for i in 0..count {
            let spawn_time = delay + seconds_per_beat / 4.0 * i as f64 - OBJ_TIME;
            let arrive_time = spawn_time + OBJ_TIME;
            let mut pos: u32 = range_rng(0, 6);
            while last_pos == pos {
                pos = range_rng(0, 6);
            }
            last_pos = pos;
            let spawn_x: f32 = range_rng(INNER_WINDOW_X_MIN, INNER_WINDOW_X_MAX);
            if let Some(object) = Object::new(
                spawn_time, arrive_time, 
                spawn_x, Objecttype::Normal, pos) {
                objects.push(object);
            }
        }
        for i in 0..count / 4 {
            let spawn_time = delay + seconds_per_beat * i as f64 - OBJ_TIME;
            let arrive_time = spawn_time + OBJ_TIME;
            let pos: u32 = range_rng(0, 2);
            let spawn_x: f32 = range_rng(INNER_WINDOW_X_MIN, INNER_WINDOW_X_MAX);
            if let Some(object) = Object::new(
                spawn_time, arrive_time, 
                spawn_x, Objecttype::Top, pos) {
                objects.push(object);
            }
        }
        objects.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());
        Self {
            objects,
            current: 0,
            song_audio,
            playing: false,
            song_start_time: 0.0,
        }
    }

    pub fn current_object(&self) -> Option<&Object> {
        self.objects.get(self.current as usize)
    }
}