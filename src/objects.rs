use bevy::prelude::*;
use crate::consts::*;
use crate::coords::Coord2d;

/// Keeps the textures and materials
#[derive(Resource)]
pub struct ObjTexture {
    pub red_obj: Handle<Image>,
    pub top_obj: Handle<Image>,
    pub vertical_obj: Handle<Image>,
    pub chain: Handle<Image>,
    pub glow: Handle<Image>,
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
    /// for long notes
    pub duration: Option<f64>,
    pub chord: bool,
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
               spawn_x: f32, objtype: Objecttype, pos: u32,
               duration: Option<f64>, chord: bool) -> Self {
        let object = Self {
            spawn_time, arrive_time,
            spawn: (spawn_x, SPAWN_POSITION).into(),
            pos, objtype,
            dest: Self::destination(objtype, pos),
            duration,
            chord,
        };
        object
    }
}

fn load_object_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ObjTexture{
        red_obj: asset_server.load("images\\redobj.png"),
        top_obj: asset_server.load("images\\topobj.png"),
        vertical_obj: asset_server.load("images\\redvo.png"),
        chain: asset_server.load("images\\chain.png"),
        glow: asset_server.load("images\\glow.png"),
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
                Objecttype::Vertical => materials.vertical_obj.clone(),
                Objecttype::Normal => materials.red_obj.clone(),
            };
            let sprite = Sprite {
                custom_size: Some(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)),
                ..default()
            };
            if object.chord {
                commands.spawn(SpriteBundle {
                    sprite,
                    texture,
                    transform,
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        texture: materials.glow.clone(),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                        ..default()
                    });
                }).insert(*object);
            } else {
                commands.spawn(SpriteBundle {
                    sprite,
                    texture,
                    transform,
                    ..default()
                }).insert(*object);
            }
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
            commands.entity(e).despawn_recursive();
            continue;
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