use crate::objects::*;
use crate::fumen::*;
use crate::consts::*;
use crate::chains::*;
use crate::utils::range_rng;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
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

        let metadata = FumenMetadata {
            name: self.name,
            artist: self.artist,
            charter: self.charter,
            bpm: bpm,
            difficulty: "Normal".to_string(),
            level: 1,
        };

        let mut objects = vec![];
        let mut chains = vec![];
        let mut chain_pos = HashMap::new();
        let mut chain_prev = HashMap::new();
        let mut chain_spawn = HashMap::new();

        let occupy_duration = seconds_from_beat(0, 1.0 / 16.0, bpm, 1e-12);
        let mut occupied = [0; BOTTOM_SLOT_COUNT as usize];
        // Stores occupations, scanning line method
        let mut occupy_events = vec![];

        // To avoid conflicts of future VOs
        let mut vo_times = vec![];
        for object in self.objects.iter() {
            if object.object_type == Objecttype::Vertical {
                let (measure, beat) = (object.measure, object.beat);
                let arrive_time = seconds_from_beat(measure, beat, bpm, delay);
                let pos = object.pos.unwrap();
                let duration = object.duration.map(|x| seconds_from_beat(0, x, bpm, 0.));
                vo_times.push((arrive_time, pos));
                occupy_events.push((arrive_time + occupy_duration + duration.unwrap_or(0.), pos, -1));
                occupy_events.push((arrive_time - occupy_duration, pos, 1));
            }
        }

        // enumerate in order of spawn time
        for (id, object) in self.objects.iter().enumerate() {
            let id = id as u32;
            let (measure, beat) = (object.measure, object.beat);
            let arrive_time = seconds_from_beat(measure, beat, bpm, delay);
            let spawn_time = arrive_time - OBJ_TIME;
            let mut pos;
            let mut spawn_x;
            let duration = object.duration.map(|x| seconds_from_beat(0, x, bpm, 0.));

            // handle occured occupy events, only unoccupy events in here
            occupy_events.retain(|(time, pos, val)| {
                if *time < arrive_time {
                    occupied[*pos as usize] += val;
                    false
                } else {
                    true
                }
            });
            // handle future vo occuping events
            vo_times.retain(|(ntime, pos)| {
                if *ntime - occupy_duration < arrive_time {
                    occupied[*pos as usize] += 1;
                    occupy_events.push((*ntime + occupy_duration, *pos, -1));
                    false
                } else {
                    true
                }
            });
            
            // generate position without considering chains
            match object.object_type {
                Objecttype::Normal => {
                    pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                    if occupied.iter().position(|x| *x == 0).is_none() {
                        error!("No available slots for normal object, overlap@ measure{} beat{}!", measure, beat);
                    } else {
                        while occupied[pos as usize] > 0 {
                            pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                        }
                    }
                    spawn_x = range_rng(SPAWN_X_MIN, SPAWN_X_MAX);
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

            // handle chained positions
            if let Some(chainedpos) = chain_pos.get(&id) {
                pos = *chainedpos;
            }
            if let Some(chainedspawn) = chain_spawn.get(&id) {
                spawn_x = *chainedspawn;
            }

            // generate position for start of chain
            if let Some(mut next) = object.chained {
                // start of chain, make sure future chain dosen't overlap with verticals
                if object.object_type == Objecttype::Normal && chain_pos.get(&id).is_none() {
                    while let Some(nobject) = self.objects.get(next as usize) {
                        if let Some(nnext) = nobject.chained {
                            next = nnext;
                        } else {
                            break;
                        }
                    }
                    let endobject = self.objects.get(next as usize).unwrap();
                    let (emeasure, ebeat) = (endobject.measure, endobject.beat);
                    let end_time = seconds_from_beat(emeasure, ebeat, bpm, delay);

                    let mut vo_occupied = [0; BOTTOM_SLOT_COUNT as usize];
                    for (ntime, pos) in vo_times.iter() {
                        if *ntime < end_time + occupy_duration &&
                            *ntime > arrive_time - occupy_duration {
                            vo_occupied[*pos as usize] += 1;
                        }
                    }
                    if occupied.iter().zip(vo_occupied.iter()).position(|(x, y)| *x + *y == 0).is_none() {
                        error!("No available slots for chain, overlap@ measure{} beat{}!", measure, beat);
                    } else {
                        while occupied[pos as usize] + vo_occupied[pos as usize] > 0 {
                            pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                        }
                    }
                }
            }

            if let Some(duration) = duration {
                // generate position for LO
                if object.object_type == Objecttype::Normal {
                    let end_time = arrive_time + duration;

                    let mut vo_occupied = [0; BOTTOM_SLOT_COUNT as usize];
                    for (ntime, pos) in vo_times.iter() {
                        if *ntime < end_time + occupy_duration &&
                            *ntime > arrive_time - occupy_duration {
                            vo_occupied[*pos as usize] += 1;
                        }
                    }
                    if occupied.iter().zip(vo_occupied.iter()).position(|(x, y)| *x + *y == 0).is_none() {
                        error!("No available slots for chain, overlap@ measure{} beat{}!", measure, beat);
                    } else {
                        while occupied[pos as usize] + vo_occupied[pos as usize] > 0 {
                            pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                        }
                    }
                }
            }

            // is it a chord?
            let chord = {
                let chord_last = if id == 0 {
                    false
                } else {
                    let last_object = self.objects.get(id as usize - 1).unwrap();
                    last_object.measure == measure && last_object.beat == beat
                };
                let chord_next = 
                if let Some(next_object) = self.objects.get(id as usize + 1) {
                    next_object.measure == measure && next_object.beat == beat
                } else {
                    false
                };
                chord_last || chord_next
            };

            objects.push(Object::new(
                spawn_time, arrive_time, 
                spawn_x, 
                object.object_type,
                pos, duration,
                chord,
            ));

            // add occupation event for itself, chain and VO events are already added
            match object.object_type {
                x if x == Objecttype::Normal || x == Objecttype::Vertical => {
                    occupied[pos as usize] += 1;
                    occupy_events.push(
                        (arrive_time + occupy_duration + duration.unwrap_or(0.), pos, -1)
                    );
                },
                _ => {},
            }

            // chains for rendering
            if let Some(prev) = chain_prev.get(&id) {
                chains.push(Chain{
                    head: objects[*prev as usize],
                    tail: objects[id as usize],
                });
            }

            // chain info for future objects
            if let Some(next) = object.chained {
                chain_pos.insert(next, pos);
                chain_prev.insert(next, id);
                chain_spawn.insert(next, spawn_x);
                // chains occupy the same position
                if object.object_type != Objecttype::Top {
                    occupied[pos as usize] += 1;
                    let nobject = &self.objects[next as usize];
                    let ntime = seconds_from_beat(nobject.measure, nobject.beat, bpm, delay);
                    occupy_events.push(
                        (ntime + occupy_duration, pos, -1)
                    );
                }
            }
        }

        // sort chains by spawn time, objects are naturally sorted
        chains.sort_by(
            |a, b| a.head.arrive_time.partial_cmp(&b.head.arrive_time).unwrap()
        );

        Fumen {
            metadata,
            objects,
            chains,
            current: 0,
            currentchain: 0,
            song_audio,
            playing: false,
            song_start_time: 0.0,
            seconds_per_measure: seconds_from_beat(1, 0.0, bpm, 0.0),
            delay,
        }
    }
}