use crate::consts::*;
use crate::utils::*;
use crate::parse::*;
use crate::objects::*;
use crate::fumen::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[allow(non_snake_case)]
pub fn INORI(asset_server: &AssetServer) -> Fumen {
    let mut objects = vec![];
    let mut id = 0;
    for measure in 0..1 {
        for i in 0..16 {
            let mut object_type = Objecttype::Normal;
            let duration = None;
            let mut pos = None;
            if i % 2 == 1 {
                object_type = Objecttype::Top;
                pos = Some(range_rng(0, TOP_SLOT_COUNT - 1));
            }
            let chained = if i % 2 == 0 && i + 2 < 16 {
                Some(id + 2)
            } else {
                None
            };
            let object = ObjectDescription {
                measure,
                beat: 1.0 / 16.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object);
            id += 1;
        }
    }

    let cycle = [999, 4];
    for measure in 1..2 {
        for i in 0..16 {
            let object_type = Objecttype::Vertical;
            let duration = None;
            let mut pos = cycle[i % 2];
            if pos == 999 {
                pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                while pos == 4 {
                    pos = range_rng(0, BOTTOM_SLOT_COUNT - 1);
                }
            }
            let pos = Some(pos);
            let chained = if cycle[i % 2] == 4 && i + 2 < 16 {
                Some(id + 2)
            } else {
                None
            };
            let object = ObjectDescription {
                measure,
                beat: 1.0 / 16.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object);
            id += 1;
        }
    }

    for measure in 2..3 {
        for i in 0..16 {
            let object_type = Objecttype::Normal;
            let duration = None;
            let pos = None;
            let chained = if (id + 3) / 8 == id / 8 {
                Some(id + 3)
            } else {
                None
            };
            let object = ObjectDescription {
                measure,
                beat: 1.0 / 16.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object);
            id += 1;
        }
    }

    for measure in 3..4 {
        for i in 0..16 {
            let object_type = Objecttype::Normal;
            let duration = None;
            let pos = None;
            let chained = if (id + 4) / 8 == id / 8 {
                Some(id + 4)
            } else {
                None
            };
            let object = ObjectDescription {
                measure,
                beat: 1.0 / 16.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object);
            id += 1;
        }
    }

    let mut chained_id = HashMap::new();
    for measure in 4..83 {
        for i in 0..16 {
            let mut object_type = match range_rng(0, 10) {
                x if x < 5 => Objecttype::Normal,
                x if x < 9 => Objecttype::Top,
                _ => Objecttype::Vertical,
            };
            if let Some(prev) = chained_id.get(&id) {
                object_type = objects[*prev as usize].object_type;
            }
            let duration = None;
            let pos = match object_type {
                Objecttype::Normal => None,
                Objecttype::Top => Some(range_rng(0, TOP_SLOT_COUNT - 1)),
                Objecttype::Vertical => Some(range_rng(0, BOTTOM_SLOT_COUNT - 1)),
            };
            let chained = if measure < 82 {
                let rng = range_rng(0, 10);
                if rng > 0 && rng < 5 {
                    let rng = rng;
                    chained_id.insert(id + rng, id);
                    Some(id + rng)
                } else {
                    None
                }
            } else {
                None
            };
            let object = ObjectDescription {
                measure,
                beat: 1.0 / 16.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object);
            id += 1;
        }
    }

    let fumen = FumenDescription {
        name: "INORI".to_string(),
        artist: "dj TAKA feat. HAL".to_string(),
        charter: "A79".to_string(),
        bpm: 146.0,
        delay: 2.0,
        objects,
    };
    fumen.into_fumen(asset_server)
}