use crate::objects::*;
use bevy::prelude::*;
use super::jsonparse::SOPoint;

#[derive(Debug, Clone)]
pub struct ObjectDescription {
    pub id: u32,
    /// spawn time in ms, will be parsed before using measure and beat
    pub starttime: f64,
    pub flytime: f64,
    /// Normal, Top, Vertical, Slide
    pub object_type: Objecttype,
    /// for long objects
    pub duration: Option<f64>,
    /// for top or vertical objects or set objects
    pub pos: Option<u32>,
    /// pos after generation
    pub generated_pos: Option<u32>,
    /// stores id of next chained object
    pub chained: Option<u32>,
    /// source of reflected object
    pub source: Option<u32>,
    pub side: u32,
}

impl ObjectDescription {
    pub fn arrive_time(&self) -> f64 {
        self.starttime + self.flytime
    }
}

/// This should be parsed from a file, then converted to the inner Fumen struct
pub struct FumenDescription {
    /// song name
    pub name: String,
    pub artist: String,
    pub charter: String,
    pub level: u32,
    pub difficulty: String,
    pub bpm: Vec<f64>,
    pub delay: f64,
    pub objects: Vec<ObjectDescription>,
    pub sopoints: Vec<SOPoint>,
}

fn seconds_from_beat(measure: u32, beat: f64, bpm: f64, delay: f64) -> f64 {
    let seconds_per_measure: f64 = 60.0 / bpm * 4.0;
    seconds_per_measure * measure as f64 + seconds_per_measure * beat + delay
}

// avoid generating in the same position for adjacent notes in this beat window
const AVOID_WINDOW: f64 = 0.5;
impl FumenDescription {
    pub fn objects_valid(&self) -> bool {
        let mut prev_time = -1e9;
        for object in self.objects.iter() {
            if object.arrive_time() < prev_time - 1e-8 {
                error!("object arrive time not in order: {:?} < {:?}", object.arrive_time(), prev_time);
                return false;
            }
            prev_time = object.arrive_time();
        }
        true
    }

    pub fn next_object_pos(&self, id: u32) -> Vec<Option<u32>> {
        let objectnow = self.objects.get(id as usize).unwrap();
        let arrive_time = objectnow.arrive_time();
        let mut next_arrive_time = None;
        for object in self.objects.iter() {
            if objectnow.side == object.side && object.arrive_time() > arrive_time + 1e-12 {
                next_arrive_time = Some(object.arrive_time());
                break;
            }
        }
        let mut next_pos = vec![];
        if !next_arrive_time.is_some() ||
            next_arrive_time.unwrap() - arrive_time > 
            seconds_from_beat(0, AVOID_WINDOW, self.bpm[0], 0.) {
            return next_pos;
        }
        for object in self.objects.iter() {
            if objectnow.side == object.side && 
                (object.arrive_time() - next_arrive_time.unwrap()).abs() < 1e-12 && 
                object.object_type != Objecttype::Top {
                next_pos.push(object.pos);
            }
        }
        next_pos
    }

    /// objects of the last time window and the chorded ones
    /// generated_pos are the position generated before
    pub fn last_object_pos(&self, id: u32, generated_pos: &Vec<u32>) -> Vec<Option<u32>> {
        let objectnow = self.objects.get(id as usize).unwrap();
        let arrive_time = objectnow.arrive_time();
        let mut last_arrive_time = None;
        for object in self.objects.iter() {
            if objectnow.side == object.side && object.arrive_time() < arrive_time - 1e-12 {
                last_arrive_time = Some(object.arrive_time());
            }
        }
        let mut last_pos = vec![];
        // chorded
        for (id, object )in self.objects.iter().enumerate() {
            if objectnow.side == object.side && 
                (object.arrive_time() - arrive_time).abs() < 1e-12 && 
                object.object_type != Objecttype::Top && 
                id < generated_pos.len() {
                last_pos.push(Some(generated_pos[id]));
            }
        }
        if !last_arrive_time.is_some() ||
            arrive_time - last_arrive_time.unwrap() > 
            seconds_from_beat(0, AVOID_WINDOW, self.bpm[0], 0.) {
            return last_pos;
        }
        for (id, object )in self.objects.iter().enumerate() {
            if objectnow.side == object.side && 
                (object.arrive_time() - last_arrive_time.unwrap()).abs() < 1e-12 && 
                object.object_type != Objecttype::Top {
                last_pos.push(Some(generated_pos[id]));
            }
        }
        last_pos
    }

    // because LO ending dosen't correspond to an object
    pub fn next_object_pos_raw(&self, arrive_time: f64, side: u32) -> Vec<Option<u32>> {
        let mut next_arrive_time = None;
        for object in self.objects.iter() {
            if side == object.side && object.arrive_time() > arrive_time + 1e-12 {
                next_arrive_time = Some(object.arrive_time());
                break;
            }
        }
        let mut next_pos = vec![];
        if !next_arrive_time.is_some() ||
            next_arrive_time.unwrap() - arrive_time > 
            seconds_from_beat(0, AVOID_WINDOW, self.bpm[0], 0.) {
            return next_pos;
        }
        for object in self.objects.iter() {
            if side == object.side && 
                (object.arrive_time() - next_arrive_time.unwrap()).abs() < 1e-12 && 
                object.object_type != Objecttype::Top {
                next_pos.push(object.pos);
            }
        }
        next_pos
    }

    pub fn last_object_id(&self, id: u32) -> Option<u32> {
        for i in (0..id).rev() {
            let object = &self.objects[i as usize];
            if object.side == self.objects[id as usize].side {
                return Some(i);
            }
        }
        None
    }

    pub fn next_object_id(&self, id: u32) -> Option<u32> {
        for i in (id + 1)..self.objects.len() as u32 {
            let object = &self.objects[i as usize];
            if object.side == self.objects[id as usize].side {
                return Some(i);
            }
        }
        None
    }
}