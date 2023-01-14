use super::jsonparse::*;
use serde_json::Value;

unsafe fn gi(arr: &mut *const u8, len: u32) -> i32 {
    unsafe {
        let ret = match len {
        1 => *(*arr as *const i8) as i32,
        2 => *(*arr as *const i16) as i32,
        4 => *(*arr as *const i32) as i32,
        _ => panic!("Invalid length"),
        };
        *arr = arr.offset(len as isize);
        ret
    }
}

unsafe fn gf(arr: &mut *const u8, len: u32) -> f32 {
    unsafe {
        let ret = match len {
        4 => *(*arr as *const f32),
        _ => panic!("Invalid length"),
        };
        *arr = arr.offset(len as isize);
        ret
    }
}

impl JsonFumen {
    pub fn from_ply(filename: &str) -> Self {
        let ply = std::fs::read(filename).unwrap();
        let mut arr = ply.as_ptr();
        unsafe {
            let header = gi(&mut arr, 4);
            let r#type = gi(&mut arr, 1);
            arr = arr.offset(11);
            let startbpm = gf(&mut arr, 4);
            let length = gi(&mut arr, 4);
            arr = arr.offset(4);
            let numnotes = gi(&mut arr, 2);
            let numbpmchanges = gi(&mut arr, 2);
            let numgeneratednotes = gi(&mut arr, 2);
            arr = arr.offset(2);
            let numsopoints = gi(&mut arr, 2);
            arr = arr.offset(6);
            let mut notes = Vec::new();
            for _ in 0..numnotes {
                let starttime = gi(&mut arr, 4);
                let flytime = gi(&mut arr, 4);
                let id = gi(&mut arr, 2);
                let source = gi(&mut arr, 2);
                let numreflectnotes = gi(&mut arr, 2);
                let mut reflecnotes = vec![];
                for _ in 0..numreflectnotes {
                    let reflectnote = gi(&mut arr, 2);
                    reflecnotes.push(reflectnote);
                }
                let numalsoreflectednotes = gi(&mut arr, 1);
                let side = gi(&mut arr, 1);
                let istop = gi(&mut arr, 1);
                let r#type = gi(&mut arr, 1);
                let lolength = gi(&mut arr, 2);
                let position = gi(&mut arr, 2);
                let isset = gi(&mut arr, 1);
                arr = arr.offset(3);
                let magic = gi(&mut arr, 1);
                arr = arr.offset(11);

                let (chainlast, chainnext, chainid, chaindelta) = 
                    if magic as u8 & 8 == 8 {
                        let chainlast = gi(&mut arr, 2);
                        let chainnext = gi(&mut arr, 2);
                        let chainid = gi(&mut arr, 2);
                        let chaindelta = gi(&mut arr, 2);
                        arr = arr.offset(4);
                        (chainlast, chainnext, chainid, chaindelta)
                    } else {
                        (0, 0, 0, 0)
                    };
                
                let mut map = serde_json::Map::new();
                map.insert("starttime".to_string(), Value::Number(serde_json::Number::from(starttime)));
                map.insert("flytime".to_string(), Value::Number(serde_json::Number::from(flytime)));
                map.insert("id".to_string(), Value::Number(serde_json::Number::from(id)));
                map.insert("source".to_string(), Value::Number(serde_json::Number::from(source)));
                map.insert("numreflectnotes".to_string(), Value::Number(serde_json::Number::from(numreflectnotes)));
                map.insert("reflectnotes".to_string(), Value::Array(reflecnotes.into_iter().map(|x| Value::Number(serde_json::Number::from(x))).collect()));
                map.insert("numalsoreflectednotes".to_string(), Value::Number(serde_json::Number::from(numalsoreflectednotes)));
                map.insert("side".to_string(), Value::Number(serde_json::Number::from(side)));
                map.insert("istop".to_string(), Value::Number(serde_json::Number::from(istop)));
                map.insert("type".to_string(), Value::Number(serde_json::Number::from(r#type)));
                map.insert("lolength".to_string(), Value::Number(serde_json::Number::from(lolength)));
                map.insert("position".to_string(), Value::Number(serde_json::Number::from(position)));
                map.insert("isset".to_string(), Value::Number(serde_json::Number::from(isset)));
                map.insert("magicnumber".to_string(), Value::Number(serde_json::Number::from(magic)));
                map.insert("chainlastid".to_string(), Value::Number(serde_json::Number::from(chainlast)));
                map.insert("chainnextid".to_string(), Value::Number(serde_json::Number::from(chainnext)));
                map.insert("chainid".to_string(), Value::Number(serde_json::Number::from(chainid)));
                map.insert("chaindeltatime".to_string(), Value::Number(serde_json::Number::from(chaindelta)));
                notes.push(Value::Object(map));
            }

            let mut bpmchanges = vec![];
            for _ in 0..numbpmchanges {
                arr = arr.offset(2);
                let id = gi(&mut arr, 2);
                let time = gi(&mut arr, 4);
                arr = arr.offset(8);
                let event = gf(&mut arr, 4);
                arr = arr.offset(16);
                bpmchanges.push(Value::Object(serde_json::Map::from_iter(vec![
                    ("id".to_string(), Value::Number(serde_json::Number::from(id))),
                    ("time".to_string(), Value::Number(serde_json::Number::from(time))),
                    ("event".to_string(), Value::Number(serde_json::Number::from_f64(event as f64).unwrap())),
                ])));
            }

            let mut sopoints = vec![];
            for _ in 0..numsopoints {
                let noteid = gi(&mut arr, 2);
                let id = gi(&mut arr, 2);
                let position = gi(&mut arr, 2);
                arr = arr.offset(2);
                let starttime = gi(&mut arr, 4);
                let flytime = gi(&mut arr, 4);
                sopoints.push(Value::Object(serde_json::Map::from_iter(vec![
                    ("noteid".to_string(), Value::Number(serde_json::Number::from(noteid))),
                    ("id".to_string(), Value::Number(serde_json::Number::from(id))),
                    ("position".to_string(), Value::Number(serde_json::Number::from(position))),
                    ("starttime".to_string(), Value::Number(serde_json::Number::from(starttime))),
                    ("flytime".to_string(), Value::Number(serde_json::Number::from(flytime))),
                ])));
            }

            JsonFumen {
                header: header as u32,
                r#type: r#type as u32,
                startbpm: startbpm as f64,
                length: length as u32,
                numnotes: numnotes as u32,
                numbpmchanges: numbpmchanges as u32,
                numgeneratednotes: numgeneratednotes as u32,
                numsopoints: numsopoints as u32,
                notes,
                bpmchanges,
                sopoints,
            }
        }
    }
}