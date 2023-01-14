use crate::coords::*;
use crate::jsonparse::SOPoint;
use crate::objects::*;
use crate::consts::*;
use crate::chains::*;
use crate::slider::SlidePoint;
use crate::slider::Slider;
use crate::utils::range_rng;
use super::fumen::*;
use super::parse::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn seconds_from_beat(measure: u32, beat: f64, bpm: f64, delay: f64) -> f64 {
    let seconds_per_measure: f64 = 60.0 / bpm * 4.0;
    seconds_per_measure * measure as f64 + seconds_per_measure * beat + delay
}

// avoid generating in the same position for adjacent notes in this beat window
const AVOID_WINDOW: f64 = 0.5;
impl FumenDescription {
    /// Need to make sure the ObjectDescription vector is sorted by spawn time
    pub fn into_fumen(&mut self, songpath: &str, song_offset: f64, asset_server: &AssetServer) -> Fumen {
        // ensure objects are sorted by arrive time
        if !self.objects_valid() {
            panic!("Objects are not sorted by arrive time!");
        }
        let delay = self.delay;
        for object in self.objects.iter_mut()  {
            object.starttime += delay;
        }

        for sopoint in self.sopoints.iter_mut() {
            sopoint.starttime += delay;
        }

        let song_audio = asset_server.load(songpath);
        let bpm: f64 = self.bpm[0];
        let (minbpm, maxbpm) = {
            let mut minbpm = bpm;
            let mut maxbpm = bpm;
            for bpm in self.bpm.iter() {
                if *bpm < minbpm {
                    minbpm = *bpm;
                }
                if *bpm > maxbpm {
                    maxbpm = *bpm;
                }
            }
            (minbpm, maxbpm)
        };

        let metadata = FumenMetadata {
            name: self.name.clone(),
            artist: self.artist.clone(),
            charter: self.charter.clone(),
            bpm: format!("{}-{}", minbpm, maxbpm),
            difficulty: self.difficulty.clone(),
            level: self.level,
        };

        let mut objects : Vec<Object>= vec![];
        let mut chains = vec![];
        let mut chain_pos = HashMap::new();
        let mut chain_prev = HashMap::new();
        let mut chain_spawn = HashMap::new();

        // both sides are parsed together since they depend on each other for refleced objects
        let mut occupied = [[0; BOTTOM_SLOT_COUNT as usize], [0; BOTTOM_SLOT_COUNT as usize]];
        // Stores occupations for previous chains and LOs, scanning line method
        // (timestamp, pos, delta) -> occupy[pos] += delta after timestamp
        let mut occupy_events = [vec![], vec![]];

        // To avoid conflicts of future VOs
        // LONG VO duration dosen't matter here since it's only for future VOs
        let mut vo_times = [vec![], vec![]];
        for object in self.objects.iter() {
            if object.object_type == Objecttype::Vertical {
                let arrive_time = object.arrive_time();
                let pos = object.pos.unwrap();
                vo_times[object.side as usize].push((arrive_time, pos));
            }
        }

        let mut generated_pos = vec![];
        let mut reflect_pos = HashMap::new();

        // enumerate in order of spawn time
        for (id, object) in self.objects.iter().enumerate() {
            // LO endings occupy for this duration, may not be accurate
            let occupy_duration = seconds_from_beat(0, AVOID_WINDOW, bpm, 1e-12);

            let id = id as u32;
            let mut pos;
            let mut spawn_x;
            let mut spawn_y = SPAWN_Y_POSITION;
            let duration = object.duration;
            let spawn_time = object.starttime;
            let arrive_time = object.arrive_time();
            let side = object.side as usize;
            let objtype = object.object_type;

            // handle occured occupy events, only unoccupy events in here
            occupy_events[side].retain(|(time, pos, val)| {
                if *time < arrive_time {
                    occupied[side][*pos as usize] += val;
                    false
                } else {
                    true
                }
            });
            
            // ========================== position generation ==========================

            // generate position without considering chains or LO or VO
            match objtype {
                Objecttype::Normal => {
                    // avoid next VOs
                    let mut adj_occupied = [0; BOTTOM_SLOT_COUNT as usize];
                    if let Some(spos) = object.pos {
                        pos = spos;
                    } else {
                        for pos in self.next_object_pos(id) {
                            if let Some(pos) = pos {
                                adj_occupied[pos as usize] += 1;
                            }
                        }
                        // avoid previous objects
                        for pos in self.last_object_pos(id, &generated_pos) {
                            if let Some(pos) = pos {
                                adj_occupied[pos as usize] += 1;
                            }
                        }
                        pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                        if occupied[side].iter().zip(adj_occupied.iter()).position(|(x, y)| *x + *y == 0).is_none() {
                            error!("No available slots for normal object, overlap@ time{:?}!", arrive_time);
                        } else {
                            while occupied[side][pos as usize] + adj_occupied[pos as usize] > 0 {
                                pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                            }
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
                Objecttype::Slide => {
                    assert!(object.pos.is_some() && object.pos.unwrap() < BOTTOM_SLOT_COUNT);
                    pos = object.pos.unwrap();
                    spawn_x = BOTTOM_SLOT_START_X + BOTTOM_SLOT_SPACING * pos as f32;
                }
            }

            // generate position for start of chain
            if let Some(mut next) = object.chained {
                // start of chain, make sure future chain dosen't overlap with verticals
                if objtype == Objecttype::Normal && chain_pos.get(&id).is_none() {
                    while let Some(nobject) = self.objects.get(next as usize) {
                        if let Some(nnext) = nobject.chained {
                            next = nnext;
                        } else {
                            break;
                        }
                    }
                    let endobject = self.objects.get(next as usize).unwrap();
                    let end_time = endobject.arrive_time();

                    let mut vo_occupied = [0; BOTTOM_SLOT_COUNT as usize];
                    // VOs during the chain
                    for (ntime, pos) in vo_times[side].iter() {
                        if *ntime <= end_time &&
                            *ntime >= arrive_time {
                            vo_occupied[*pos as usize] += 1;
                        }
                    }
                    // the next objects of end of chain
                    for pos in self.next_object_pos(next) {
                        if let Some(pos) = pos {
                            vo_occupied[pos as usize] += 1;
                        }
                    }
                    // sum of previous occupy and future VO occupy
                    if occupied[side].iter().zip(vo_occupied.iter()).position(|(x, y)| *x + *y == 0).is_none() {
                        error!("No available slots for normal object, overlap@ time{:?}!", arrive_time);
                    } else {
                        while occupied[side][pos as usize] + vo_occupied[pos as usize] > 0 {
                            pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                        }
                    }
                }
            }

            // generate position for LO, which can't be the start of a chain
            if let Some(duration) = duration {
                if objtype == Objecttype::Normal {
                    // works the same as chains
                    let end_time = arrive_time + duration;
                    let mut vo_occupied = [0; BOTTOM_SLOT_COUNT as usize];
                    for (ntime, pos) in vo_times[side].iter() {
                        if *ntime <= end_time &&
                            *ntime >= arrive_time {
                            vo_occupied[*pos as usize] += 1;
                        }
                    }
                    for pos in self.next_object_pos_raw(
                        end_time, side as u32
                    ) {
                        if let Some(pos) = pos {
                            vo_occupied[pos as usize] += 1;
                        }
                    }
                    if occupied[side].iter().zip(vo_occupied.iter()).position(|(x, y)| *x + *y == 0).is_none() {
                        error!("No available slots for normal object, overlap@ time{:?}!", arrive_time);
                    } else {
                        while occupied[side][pos as usize] + vo_occupied[pos as usize] > 0 {
                            pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                        }
                    }
                }
            }

            // handle chained positions last, overwrites previous results
            if let Some(chainedpos) = chain_pos.get(&id) {
                pos = *chainedpos;
            }

            // handle set objects
            if let Some(spos) = object.pos {
                pos = spos;
            }

            generated_pos.push(pos);

            // ========================== spawn position and reflect generation ==========================

            let mut reflect: Option<(f32, f32)> = None;

            // assume all notes aren't missed
            if let Some(source) = object.source {
                let sourceobj = objects.get(source as usize).unwrap();
                (spawn_x, spawn_y) = sourceobj.dest.into();
                // generated tops donsen't reflect
                if objtype != Objecttype::Top {
                    if let Some((rx, ry)) = reflect_pos.get(&source) {
                        reflect = Some((*rx, *ry));
                    } else {
                        // TODO: check if the distribution is actually this
                        let reflec_y = range_rng(REFLECT_Y_MIN, REFLECT_Y_MAX);
                        let dest = Object::destination(objtype, pos, side as u32);
                        // choose the shortest path
                        let p1: Coord2d = (REFLECT_X_LEFT, reflec_y).into();
                        let p2: Coord2d = (REFLECT_X_RIGHT, reflec_y).into();
                        let d1 = p1.distance(&(spawn_x, spawn_y).into()) + p1.distance(&dest);
                        let d2 = p2.distance(&(spawn_x, spawn_y).into()) + p2.distance(&dest);
                        if d1 < d2 {
                            reflect = Some(p1.into());
                        } else {
                            reflect = Some(p2.into());
                        }
                        reflect_pos.insert(source, reflect.unwrap());
                    }
                }
            } else {
                // if the object has no source, it's generated and the spawn position is already generated
                // so we only consider chain spawns here
                if let Some((chainx, chainy)) = chain_spawn.get(&id) {
                    (spawn_x, spawn_y) = (*chainx, *chainy)
                }
            }

            // ========================== finishing up ==========================

            let chord = {
                let lastid = self.last_object_id(id);
                let nextid = self.next_object_id(id);
                let lastchord = if let Some(lastid) = lastid {
                    let lastobject = self.objects.get(lastid as usize).unwrap();
                    (lastobject.arrive_time() - arrive_time).abs() < 1e-12
                } else {
                    false
                };
                let nextchord = if let Some(nextid) = nextid {
                    let nextobject = self.objects.get(nextid as usize).unwrap();
                    (nextobject.arrive_time() - arrive_time).abs() < 1e-12
                } else {
                    false
                };
                lastchord || nextchord
            };

            // to make rendering LO easier
            if reflect.is_none() {
                reflect = Some((spawn_x, spawn_y));
            }

            let result = Object::new(
                spawn_time, arrive_time,
                side as u32,
                spawn_x, spawn_y,
                objtype,
                pos,
                reflect,
                duration,
                chord,
            );
            objects.push(result);

            // chains for rendering, does not render blue side
            if side == 0 {
                if let Some(prev) = chain_prev.get(&id) {
                    // object is copied here since it derives Copy trait
                    chains.push(Chain{
                        head: objects[*prev as usize],
                        tail: objects[id as usize],
                    });
                }
            }

            // chain info for future objects
            if let Some(next) = object.chained {
                chain_pos.insert(next, pos);
                chain_prev.insert(next, id);
                chain_spawn.insert(next, (spawn_x, spawn_y));
                // chains occupy the same position
                if object.object_type != Objecttype::Top {
                    occupied[side][pos as usize] += 1;
                    let nobject = &self.objects[next as usize];
                    let ntime = nobject.arrive_time();
                    occupy_events[side].push(
                        (ntime + 1e-12, pos, -1)
                    );
                }
            }

            // occupy events for LO
            if objtype == Objecttype::Normal ||
               objtype == Objecttype::Vertical {
                if let Some(duration) = duration {
                    occupied[side][pos as usize] += 1;
                    occupy_events[side].push(
                        (arrive_time + duration + occupy_duration + 1e-12, pos, -1)
                    );
                }
            }
        }

        // sort chains by spawn time, objects are naturally sorted
        chains.sort_by(
            |a, b| a.head.arrive_time.partial_cmp(&b.head.arrive_time).unwrap()
        );

        // ========================== attaching SO points ==========================
        let mut attach: HashMap<u32, Vec<&SOPoint>> = HashMap::new();
        let mut sliders = vec![];
        for point in &self.sopoints {
            if let Some(v) = attach.get_mut(&point.noteid) {
                v.push(point);
            } else {
                attach.insert(point.noteid, vec![point]);
            }
        }

        for id in attach.keys() {
            if objects[*id as usize].side != 0 {
                continue;
            }
            let start = SlidePoint {
                pos: self.objects[*id as usize].pos.unwrap(),
                spawn_time: objects[*id as usize].spawn_time,
                arrive_time: objects[*id as usize].arrive_time,
                spawn: objects[*id as usize].spawn,
                dest: objects[*id as usize].dest,
            };
            let mut points = vec![start];
            for point in attach.get(id).unwrap() {
                let pos = point.pos;
                let spawn_time = point.starttime;
                let arrive_time = spawn_time + point.flytime;
                let spawn = (BOTTOM_SLOT_START_X + BOTTOM_SLOT_SPACING * pos as f32, SPAWN_Y_POSITION).into();
                let dest = Object::destination(
                    Objecttype::Slide,
                    pos,
                    objects[*id as usize].side,
                );
                points.push(SlidePoint {
                    pos,
                    spawn_time,
                    arrive_time,
                    spawn,
                    dest,
                });
            }
            sliders.push(Slider{
                parts: points,
            });
        }

        sliders.sort_by(
            |a, b| a.spawn_time().partial_cmp(&b.spawn_time()).unwrap()
        );

        Fumen {
            metadata,
            objects,
            current: 0,
            chains,
            currentchain: 0,
            sliders,
            currentslider: 0,
            song_audio,
            playing: false,
            song_start_time: 0.0,
            seconds_per_measure: seconds_from_beat(1, 0.0, bpm, 0.0),
            delay,
            song_offset,
            bpm,
        }
    }
}