use bevy::prelude::{error};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use super::parse::{FumenDescription, ObjectDescription};
use crate::objects::Objecttype;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    ReadError,
    InvalidJson,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        error!("{}", err);
        ParseError::ReadError
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        error!("{}", err);
        ParseError::InvalidJson
    }
}

fn parse_objtype(objtype: u32, istop: u32, isset: u32, position: i32) -> Objecttype {
    if objtype == 0 || objtype == 1 {
        if istop == 1 {
            Objecttype::Top
        } else {
            if isset == 0 && position != -1 {
                Objecttype::Vertical
            } else {
                Objecttype::Normal
            }
        }
    } else {
        Objecttype::Slide
    }
}

const NOT_FOUND_VAL: i64 = -114514;
fn get_val(object: &Value, name: &str) -> i64 {
    match object[name].as_i64() {
        Some(val) => val,
        None => {
            error!("field {} not found in json", name);
            NOT_FOUND_VAL
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFumen {
    header: u32,
    r#type: u32,
    startbpm: f64,
    length: u32,
    numnotes: u32,
    numbpmchanges: u32,
    numgeneratednotes: u32,
    numsopoints: u32,
    notes: Vec<Value>,
    bpmchanges: Vec<Value>,
    sopoints: Vec<Value>,
}

const MS_PER_SEC: f64 = 1000.;
impl FumenDescription {
    /// generatednotes: 非反射note数量
    /// reflectnotes: 反弹出note
    /// numalsoreflectednotes: 同时反弹球数
    /// side: 0 红 1 蓝
    /// type: 0 普通 1 LO 2 SO
    /// magic: 一个Bitflag:
    /// 4: Bothside, 32: ReflectNum > 0
    /// 16: fromNote, 1: Sim
    /// 8: Chained 2: Top & Long & Sim
    /// TODO: 啥是SIM？
    /// VO判断：没有isset，不是top并且position不是-1
    pub fn from_json(
        name: &str, artist: &str, charter: &str
    ) -> Result<Self, ParseError> {
        let json = fs::read_to_string(format!("fumens\\{}\\fumen.json", name))?;
        let content: JsonFumen = serde_json::from_str(&json)?;
        let mut objects = vec![];
        let numnotes = content.numnotes;
        // id in json -> index in my object array
        let mut idtranslate = HashMap::new();

        for i in 0..numnotes {
            let object = &content.notes[i as usize];

            let starttime: i32 = get_val(object, "starttime") as i32;
            let flytime: u32 = get_val(object, "flytime") as u32;
            let id: u32 = get_val(object, "id") as u32;
            let source: i32 = get_val(object, "source") as i32;
            let side: u32 = get_val(object, "side") as u32;
            let istop: u32 = get_val(object, "istop") as u32;
            let objtype: u32 = get_val(object, "type") as u32;
            let lolength: u32 = get_val(object, "lolength") as u32;
            let position: i32 = get_val(object, "position") as i32;
            let isset: u32 = get_val(object, "isset") as u32;
            let magic: u32 = get_val(object, "magicnumber") as u32;
            let chainlast: i32 = get_val(object, "chainlastid") as i32;
            let chainnext: i32 = get_val(object, "chainnextid") as i32;
            let numreflectnotes = get_val(object, "numreflectnotes") as i32;
            let reflectnotes = &object["reflectnotes"];
            let sametimereflects = get_val(object, "numalsoreflectednotes") as i32;

            let object_type = parse_objtype(objtype, istop, isset, position);

            if object_type == Objecttype::Slide {
                continue;
            }

            let islong = objtype == 1;
            let duration = if islong {
                Some(lolength as f64 / MS_PER_SEC)
            } else {
                None
            };
            let pos = if position == -1 {
                None
            } else {
                if object_type == Objecttype::Top {
                    // this is flipped for some reason
                    Some(2 - position as u32)
                } else {
                    Some(position as u32)
                }
            };
            let chained = if magic & 8 != 0 {
                if chainnext != -1 {
                    // this may need to be translated to the real index
                    Some(chainnext as u32)
                } else {
                    None
                }
            } else {
                None
            };
            let source = if source == -1 {
                None
            } else {
                Some(source as u32)
            };

            let objdesc = ObjectDescription {
                starttime: starttime as f64 / MS_PER_SEC,
                flytime: flytime as f64 / MS_PER_SEC,
                object_type, duration, pos,
                generated_pos: None,
                chained, source, side,
            };
            objects.push(objdesc);
            // since some objects may be omitted(Slides), we have to keep track of the actual id
            idtranslate.insert(id as u32, objects.len() - 1);
        }
        for object in objects.iter_mut() {
            object.chained = object.chained.map(|id| *idtranslate.get(&id).unwrap() as u32);
            object.source = object.source.map(|id| *idtranslate.get(&id).unwrap() as u32);
        }
        Ok(FumenDescription {
            name: name.to_string(),
            artist: artist.to_string(),
            charter: charter.to_string(),
            level: 1,
            difficulty: "Normal".to_string(),
            // TODO: change to range
            bpm: vec![content.startbpm],
            delay: -0.22,
            objects,
        })
    }
}