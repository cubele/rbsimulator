use crate::consts::*;
use crate::utils::*;
use crate::parse::*;
use crate::objects::*;
use crate::fumen::*;
use bevy::prelude::*;
use std::collections::HashMap;

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

    for measure in 4..5 {
        for i in 0..8 {
            let object_type = Objecttype::Normal;
            let duration = None;
            let pos = None;
            let chained = None;
            let mut object = ObjectDescription {
                measure,
                beat: 1.0 / 8.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object.clone());
            object.object_type = Objecttype::Top;
            object.pos = Some(range_rng(0, TOP_SLOT_COUNT - 1));
            objects.push(object.clone());
            object.object_type = Objecttype::Vertical;
            object.pos = Some(range_rng(0, BOTTOM_SLOT_COUNT - 1));
            objects.push(object);
            id += 3;
        }
    }

    for measure in 5..83 {
        let object_type = match range_rng(0, 10) {
            x if x < 5 => Objecttype::Normal,
            x if x < 9 => Objecttype::Top,
            _ => Objecttype::Vertical,
        };
        let duration = Some(1.);
        let pos = match object_type {
            Objecttype::Normal => None,
            Objecttype::Top => Some(range_rng(0, TOP_SLOT_COUNT - 1)),
            Objecttype::Vertical => Some(range_rng(0, BOTTOM_SLOT_COUNT - 1)),
        };
        let (occupy, otype) = if let Some(op) = pos {
            (op, object_type)
        } else {
            (0, Objecttype::Normal)
        };
        let chained = None;
        let object = ObjectDescription {
            measure,
            beat: 0 as f64,
            object_type,
            duration,
            pos,
            chained,
        };
        objects.push(object);
        id += 1;
        for i in 0..16 {
            let object_type = match range_rng(0, 10) {
                x if x < 5 => Objecttype::Normal,
                x if x < 9 => Objecttype::Top,
                _ => Objecttype::Vertical,
            };
            let duration = None;
            let mut pos = match object_type {
                Objecttype::Normal => None,
                Objecttype::Top => Some(range_rng(0, TOP_SLOT_COUNT - 1)),
                Objecttype::Vertical => Some(range_rng(0, BOTTOM_SLOT_COUNT - 1)),
            };
            while object_type == otype && pos == Some(occupy) {
                pos = match object_type {
                    Objecttype::Normal => None,
                    Objecttype::Top => Some(range_rng(0, TOP_SLOT_COUNT - 1)),
                    Objecttype::Vertical => Some(range_rng(0, BOTTOM_SLOT_COUNT - 1)),
                };
            }
            let chained = None;
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
        name: "INORIです".to_string(),
        artist: "dj TAKA feat. HAL".to_string(),
        charter: "A79".to_string(),
        bpm: 146.0,
        delay: 2.0,
        objects,
    };
    fumen.into_fumen(asset_server)
}

pub fn testfumen(asset_server: &AssetServer) -> Fumen {
    let mut objects = vec![];
    let mut id = 0;
    for measure in 0..1 {
        for i in 0..8 {
            let object_type = Objecttype::Normal;
            let duration = Some(1.0 / 8.0);
            let pos = None;
            let chained = None;
            let object = ObjectDescription {
                measure,
                beat: 1.0 / 8.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object);
            id += 1;
        }
    }

    for measure in 1..2 {
        for i in 0..4 {
            let object_type = Objecttype::Normal;
            let duration = Some(1.0 / 4.0);
            let pos = None;
            let chained = None;
            let mut object = ObjectDescription {
                measure,
                beat: 1.0 / 4.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object.clone());
            object.object_type = Objecttype::Top;
            object.pos = Some(range_rng(0, TOP_SLOT_COUNT - 1));
            objects.push(object.clone());
            object.object_type = Objecttype::Vertical;
            object.pos = Some(range_rng(0, BOTTOM_SLOT_COUNT - 1));
            objects.push(object);
            id += 3;
        }
    }

    let cycle = [999, 1];
    for measure in 2..3 {
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

    for measure in 3..4 {
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

    for measure in 4..5 {
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

    for measure in 5..6 {
        for i in 0..8 {
            let object_type = Objecttype::Normal;
            let duration = None;
            let pos = None;
            let chained = None;
            let mut object = ObjectDescription {
                measure,
                beat: 1.0 / 8.0 * i as f64,
                object_type,
                duration,
                pos,
                chained,
            };
            objects.push(object.clone());
            object.object_type = Objecttype::Top;
            object.pos = Some(range_rng(0, TOP_SLOT_COUNT - 1));
            objects.push(object.clone());
            object.object_type = Objecttype::Vertical;
            object.pos = Some(range_rng(0, BOTTOM_SLOT_COUNT - 1));
            objects.push(object);
            id += 3;
        }
    }

    let fumen = FumenDescription {
        name: "INORIです".to_string(),
        artist: "dj TAKA feat. HAL".to_string(),
        charter: "A79".to_string(),
        bpm: 146.0,
        delay: 2.0,
        objects,
    };
    fumen.into_fumen(asset_server)
}