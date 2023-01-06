use bevy::prelude::*;
use crate::consts::*;
use crate::coords::Coord2d;

/// Keeps the textures and materials
#[derive(Resource)]
pub struct ObjTexture {
    pub red_obj: Handle<Image>,
    pub top_obj: Handle<Image>,
    pub chain: Handle<Image>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Objecttype {
    Normal,
    Vertical,
    Top,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Object {
    pub spawn_time: f64,
    pub arrive_time: f64,
    /// reflections are not considered for now
    pub spawn: Coord2d,
    pub dest: Coord2d,
    /// dest position, [0-6] for normal, [0-2] for top
    pub pos: u32,
    pub objtype: Objecttype,
}

impl Object {
    pub fn destination(objtype: Objecttype, pos: u32) -> Coord2d {
        let x = match objtype {
            Objecttype::Top => TOP_SLOT_START_X + TOP_SLOT_SPACING * pos as f32,
            _ => BOTTOM_SLOT_START_X + BOTTOM_SLOT_SPACING * pos as f32,
        };
        let y = match objtype {
            Objecttype::Top => TOP_SLOT_Y,
            _ => BOTTOM_SLOT_Y,
        };
        Coord2d::new(x, y)
    }

    pub fn current_coord(&self, time: f64) -> Coord2d {
        let t = (time - self.spawn_time) / (self.arrive_time - self.spawn_time);
        self.spawn + (self.dest - self.spawn) * t as f32
    }

    pub fn new(spawn_time: f64, arrive_time: f64,
               spawn_x: f32, objtype: Objecttype, pos: u32) -> Option<Self> {
        match objtype {
            Objecttype::Normal => {
                if pos > 6 {
                    error!("Invalid Normal object position: {} @ {}", pos, spawn_time);
                    return None;
                }
            }
            Objecttype::Vertical => {
                if pos > 6 {
                    error!("Invalid Vertical object position: {} @ {}", pos, spawn_time);
                    return None;
                }
            }
            Objecttype::Top => {
                if pos > 2 {
                    error!("Invalid Top object position: {} @ {}", pos, spawn_time);
                    return None;
                }
            }
        }
        let object = Self {
            spawn_time, arrive_time,
            spawn: Coord2d::new(spawn_x, SPAWN_POSITION),
            pos, objtype,
            dest: Self::destination(objtype, pos),
        };
        Some(object)
    }
}

fn load_object_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ObjTexture{
        red_obj: asset_server.load("images\\sad.png"),
        top_obj: asset_server.load("images\\topobject.png"),
        chain: asset_server.load("images\\chain.png"),
    });
}

use crate::fumen::Fumen;
fn spawn_objects(
    mut commands: Commands,
    mut fumen: ResMut<Fumen>,
    materials: Res<ObjTexture>,
    time: Res<Time>,
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    while let Some(object) = fumen.current_object() {
        if object.spawn_time < time_now {
            let transform = object.current_coord(time_now)
                                             .into_transform(OBJECT_Z + fumen.current as f32 * OBJECT_Z_DIFF);
            let texture = match object.objtype {
                Objecttype::Top => materials.top_obj.clone(),
                _ => materials.red_obj.clone(),
            };
            commands.spawn(SpriteBundle {
                texture,
                transform,
                ..default()
            })
            .insert(*object);
            info!("Spawned object: {:?}", object);
            fumen.current += 1;
        } else {
            break;
        }
    }
}

use crate::sfx::SoundFX;
/// also plays sfx for low latency
fn move_objects(mut commands: Commands, time: Res<Time>,
                mut query: Query<(Entity, &mut Transform, &Object)>,
                audio: Res<Audio>, sfx: Res<SoundFX>, fumen: Res<Fumen>) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    for (e,
        mut transform,
        object) in query.iter_mut() {
        // passed the judgement line
        if transform.translation.y < object.dest.y() {
            audio.play_with_settings(
                sfx.justsound.clone(),
                PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
            );
            commands.entity(e).despawn();
        }
        (transform.translation.x, transform.translation.y) = 
            object.current_coord(time_now).into();
    }
}

use crate::chains::*;
pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_object_texture)
            .add_system(spawn_objects)
            .add_system(move_objects)
            .add_system(spawn_chains)
            .add_system(move_chains);
    }
}