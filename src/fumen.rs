use bevy::prelude::*;
use crate::consts::*;
use crate::objects::{Object, Objecttype};
use crate::chains::Chain;
use std::sync::Arc;

#[derive(Resource, Debug)]
pub struct Fumen {
    pub objects: Vec<Object>,
    pub chains: Vec<Chain>,
    pub current: usize,
    pub currentchain: usize,
    pub song_audio: Handle<AudioSource>,
    pub playing: bool,
    pub song_start_time: f64,
}

use crate::utils::range_rng;
impl Fumen {
    pub fn dummy(asset_server: &AssetServer) -> Self {
        let delay = 2.0;
        let song_audio = asset_server.load("..\\fumens\\INORI\\song.ogg");
        let bpm: f64 = 146.0;
        // quarter note
        let seconds_per_beat: f64 = 60.0 / bpm;
        let mut objects = vec![];
        let mut chains = vec![];
        let count = 1600;
        let mut last_pos = 10;
        let mut last_x = 0.0;
        // chains
        for i in 0..(count / 16) {
            for j in 0..8 {
                let spawn_time = delay + seconds_per_beat / 2.0 * (i * 8 + j) as f64 - OBJ_TIME;
                let arrive_time = spawn_time + OBJ_TIME;
                let mut pos: u32 = range_rng(0, 6);
                let mut spawn_x: f32 = range_rng(INNER_WINDOW_X_MIN, INNER_WINDOW_X_MAX);
                if j != 0 {
                    pos = last_pos;
                    spawn_x = last_x;
                }
                last_pos = pos;
                last_x = spawn_x;
                if let Some(object) = Object::new(
                    spawn_time, arrive_time, 
                    spawn_x, Objecttype::Normal, pos) {
                    objects.push(object);
                }
                if j > 0 {
                    chains.push(Chain{
                        head: *objects.get(objects.len() - 2).unwrap(),
                        tail: *objects.get(objects.len() - 1).unwrap(),
                    });
                }
            }
        }

        // normal objects
        for i in 0..(count / 2) {
            let spawn_time = delay + seconds_per_beat / 2.0 * i as f64 + seconds_per_beat / 4.0 - OBJ_TIME;
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
        
        // top objects
        for i in 0..count / 8 {
            let spawn_time = delay + seconds_per_beat * 2.0 * i as f64 - OBJ_TIME;
            let arrive_time = spawn_time + OBJ_TIME;
            let pos: u32 = range_rng(0, 2);
            let spawn_x: f32 = TOP_SPAWN_X_START + TOP_SPAWN_X_SPACING * pos as f32;
            if let Some(object) = Object::new(
                spawn_time, arrive_time, 
                spawn_x, Objecttype::Top, pos) {
                objects.push(object);
            }
        }

        // sort by spawn time
        objects.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());
        chains.sort_by(|a, b| a.head.spawn_time.partial_cmp(&b.head.spawn_time).unwrap());
        Self {
            objects,
            chains,
            current: 0,
            currentchain: 0,
            song_audio,
            playing: false,
            song_start_time: 0.0,
        }
    }

    pub fn current_object(&self) -> Option<&Object> {
        self.objects.get(self.current as usize)
    }

    pub fn current_chain(&self) -> Option<&Chain> {
        self.chains.get(self.currentchain as usize)
    }
}