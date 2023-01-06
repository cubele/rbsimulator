use crate::objects::*;
use crate::fumen::*;
use crate::consts::*;
use crate::chains::*;
use crate::utils::range_rng;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ObjectDescription {
    pub measure: u32,
    pub beat: f64,
    /// Normal, Top, Vertical
    pub object_type: Objecttype,
    /// for long objects
    pub duration: Option<f64>,
    /// for top or vertical objects
    pub pos: Option<u32>,
    /// stores id of next chained object
    pub chained: Option<u32>
}

/// This should be parsed from a file, then converted to the inner Fumen struct
pub struct FumenDescription {
    /// song name
    pub name: String,
    pub artist: String,
    pub charter: String,
    pub bpm: f64,
    pub delay: f64,
    pub objects: Vec<ObjectDescription>,
}

fn seconds_from_beat(measure: u32, beat: f64, bpm: f64, delay: f64) -> f64 {
    let seconds_per_measure: f64 = 60.0 / bpm * 4.0;
    seconds_per_measure * measure as f64 + seconds_per_measure * beat + delay
}

impl FumenDescription {
    /// Need to make sure the ObjectDescription vector is sorted by spawn time
    pub fn into_fumen(self, asset_server: &AssetServer) -> Fumen {
        let delay = self.delay;
        let audio_path = format!("..\\fumens\\{}\\song.ogg", self.name);
        let song_audio = asset_server.load(audio_path);
        let bpm: f64 = self.bpm;

        let mut objects = vec![];
        let mut chains = vec![];
        let mut chain_pos = HashMap::new();
        let mut chain_prev = HashMap::new();
        let mut chain_spawn = HashMap::new();

        let occupy_duration = seconds_from_beat(0, 1.0 / 16.0, bpm, 1e-12);
        let mut occupied = [0; BOTTOM_SLOT_COUNT as usize];
        // Stores the end of occupation, scanning line method
        let mut occupy_events = vec![];
        for (id, object) in self.objects.iter().enumerate() {
            let id = id as u32;
            let (measure, beat) = (object.measure, object.beat);
            let arrive_time = seconds_from_beat(measure, beat, bpm, delay);
            let spawn_time = arrive_time - OBJ_TIME;
            let mut pos;
            let mut spawn_x;
            let duration = object.duration;

            occupy_events.retain(|(time, pos)| {
                if *time < arrive_time {
                    occupied[*pos as usize] -= 1;
                    false
                } else {
                    true
                }
            });
            // VO in later measures may cause conflicts
            let mut sid = id + 1;
            while let Some(nobject) = self.objects.get(sid as usize) {
                let ntime = seconds_from_beat(nobject.measure, nobject.beat, bpm, delay);
                if ntime - arrive_time > occupy_duration {
                    break;
                }
                if nobject.object_type == Objecttype::Vertical {
                    let next_pos = nobject.pos.unwrap();
                    occupied[next_pos as usize] += 1;
                    occupy_events.push((ntime + occupy_duration, next_pos));
                }
                sid += 1;
            }
            
            match object.object_type {
                Objecttype::Normal => {
                    pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                    while occupied[pos as usize] > 0 {
                        pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                    }
                    spawn_x = range_rng(INNER_WINDOW_X_MIN, INNER_WINDOW_X_MAX);
                },
                Objecttype::Top => {
                    assert!(object.pos.is_some() && object.pos.unwrap() < TOP_SLOT_COUNT);
                    pos = object.pos.unwrap();
                    spawn_x = TOP_SPAWN_X_START + TOP_SPAWN_X_SPACING * pos as f32;
                },
                Objecttype::Vertical => {
                    assert!(object.pos.is_some() && object.pos.unwrap() < BOTTOM_SLOT_COUNT);
                    pos = object.pos.unwrap();
                    spawn_x = BOTTOM_SLOT_START_X + BOTTOM_SLOT_SPACING * pos as f32;
                },
            }

            if let Some(chainedpos) = chain_pos.get(&id) {
                pos = *chainedpos;
            }

            if let Some(chainedspawn) = chain_spawn.get(&id) {
                spawn_x = *chainedspawn;
            }

            objects.push(Object::new(
                spawn_time, arrive_time, 
                spawn_x, 
                object.object_type,
                pos, duration,
            ));

            match object.object_type {
                Objecttype::Normal => {
                    occupied[pos as usize] += 1;
                    occupy_events.push(
                        (arrive_time + occupy_duration, pos)
                    );
                },
                Objecttype::Top => {
                },
                Objecttype::Vertical => {
                    occupied[pos as usize] += 1;
                    occupy_events.push(
                        (arrive_time + occupy_duration, pos)
                    );
                },
            }

            if let Some(prev) = chain_prev.get(&id) {
                chains.push(Chain{
                    head: objects[*prev as usize],
                    tail: objects[id as usize],
                });
            }

            if let Some(next) = object.chained {
                chain_pos.insert(next, pos);
                chain_prev.insert(next, id);
                chain_spawn.insert(next, spawn_x);
                // chains occupy the same position
                occupied[pos as usize] += 1;
                let nobject = &self.objects[next as usize];
                let ntime = seconds_from_beat(nobject.measure, nobject.beat, bpm, delay);
                occupy_events.push(
                    (ntime + occupy_duration, pos)
                );
            }
        }
        // sort by spawn time, objects are naturally sorted
        chains.sort_by(
            |a, b| a.head.arrive_time.partial_cmp(&b.head.arrive_time).unwrap()
        );

        Fumen {
            objects,
            chains,
            current: 0,
            currentchain: 0,
            song_audio,
            playing: false,
            song_start_time: 0.0,
        }
    }
}