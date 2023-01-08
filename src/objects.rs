use bevy::prelude::*;
use crate::consts::*;
use crate::coords::Coord2d;

/// Keeps the textures and materials
#[derive(Resource)]
pub struct ObjTexture {
    pub red_obj: Handle<Image>,
    pub red_lo_start: Handle<Image>,
    pub red_lo_mid: Handle<Image>,
    pub red_lo_end: Handle<Image>,
    pub top_obj: Handle<Image>,
    pub top_lo_start: Handle<Image>,
    pub top_lo_mid: Handle<Image>,
    pub top_lo_end: Handle<Image>,
    pub vertical_obj: Handle<Image>,
    pub vertical_lo_start: Handle<Image>,
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
        red_lo_start: asset_server.load("images\\redobj.png"),
        red_lo_mid: asset_server.load("images\\redlomiddle.png"),
        red_lo_end: asset_server.load("images\\redloend.png"),
        top_obj: asset_server.load("images\\topobj.png"),
        top_lo_start: asset_server.load("images\\topobj.png"),
        top_lo_mid: asset_server.load("images\\toplomiddle.png"),
        top_lo_end: asset_server.load("images\\toploend.png"),
        vertical_obj: asset_server.load("images\\redvo.png"),
        vertical_lo_start: asset_server.load("images\\redvo.png"),
        chain: asset_server.load("images\\chain.png"),
        glow: asset_server.load("images\\glow.png"),
    });
}

#[derive(Component)]
struct LoMid;

#[derive(Component)]
struct LoEnd;

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
            let islong = object.duration.is_some();
            let texture = match object.objtype {
                Objecttype::Top => if islong {
                    materials.top_lo_start.clone() 
                } else {
                    materials.top_obj.clone()
                },
                Objecttype::Vertical => if islong {
                    materials.vertical_lo_start.clone()
                } else {
                    materials.vertical_obj.clone()
                },
                Objecttype::Normal => if islong {
                    materials.red_lo_start.clone()
                } else {
                    materials.red_obj.clone()
                },
            };
            let sprite = Sprite {
                custom_size: Some(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)),
                ..default()
            };
            let bundle = SpriteBundle {
                sprite: sprite.clone(),
                texture,
                transform,
                ..default()
            };
            let mut e = commands.spawn(bundle);
            if let Some(duration) = object.duration {
                let (midtexture, endtexture) = match object.objtype {
                    Objecttype::Top => (materials.top_lo_mid.clone(), materials.top_lo_end.clone()),
                    Objecttype::Vertical => (materials.red_lo_mid.clone(), materials.red_lo_end.clone()),
                    Objecttype::Normal => (materials.red_lo_mid.clone(), materials.red_lo_end.clone()),
                };
                let p1 = object.current_coord(time_now);
                let p2 = object.current_coord(time_now - duration.min(0.15));
                // expand the line to a rectangle
                let w = OBJECT_SIZE;
                let h = p1.distance(&p2);
                let angle = p1.angle(&p2);
                let (mx, my) = ((p2 - p1) / 2.0).into();
                let transform_mid = Transform::from_xyz(
                    mx, my, -0.5)
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle));
                e.add_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(w, h)),
                            ..default()
                        },
                        texture: midtexture,
                        transform: transform_mid,
                        ..default()
                    }).insert(LoMid);
                });
                let (ex, ey) = (p2 - p1).into();
                let transform_end = Transform::from_xyz(ex, ey, -1.)
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle));
                e.add_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)),
                            ..default()
                        },
                        texture: endtexture,
                        transform: transform_end,
                        ..default()
                    }).insert(LoEnd);
                });
            }
            if object.chord {
                e.add_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: sprite.clone(),
                        texture: materials.glow.clone(),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                        ..default()
                    });
                });
            }
            e.insert(*object);
            fumen.current += 1;
        } else {
            break;
        }
    }
}

use crate::sfx::SoundFX;
/// also plays sfx for lower latency
fn move_objects(
    mut commands: Commands, time: Res<Time>,
    mut query_single: Query<(Entity, &mut Transform, &Object), (Without<LoMid>, Without<LoMid>, Without<Children>)>,
    mut query: Query<(Entity, &mut Transform, &Object, &Children), (Without<LoMid>, Without<LoMid>)>,
    mut query_lomid: Query<(Entity, &mut Sprite, &mut Transform), (With<LoMid>, Without<LoEnd>, Without<Object>)>,
    mut query_loend: Query<(Entity, &mut Transform), (With<LoEnd>, Without<LoMid>, Without<Object>)>,
    audio: Res<Audio>, sfx: Res<SoundFX>, fumen: Res<Fumen>
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    let time_last = time_now - time.delta_seconds_f64();
    let mut played = false;
    for (e,
        mut transform,
        object,
        children) in query.iter_mut() {
        if let Some(duration) = object.duration {
            let disp_duration = duration.min(0.15);
            // LO
            if time_now < object.arrive_time {
                (transform.translation.x, transform.translation.y) = 
                    object.current_coord(time_now).into();
            }
            if time_last < object.arrive_time && time_now >= object.arrive_time {
                // just arrived
                if !played {
                    audio.play_with_settings(
                        sfx.justsound.clone(),
                        PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                    );
                    played = true;
                }
                (transform.translation.x, transform.translation.y) = 
                    object.dest.into();
            }
            if time_now >= object.arrive_time && time_now < object.arrive_time + duration {
                // in the middle
                let p1 = object.dest;
                let duration_rem = duration - (time_now - object.arrive_time);
                let rem_percent = duration_rem / duration;
                let p2 = object.current_coord(object.arrive_time - disp_duration * rem_percent);

                let angle = p1.angle(&p2);
                let (mx, my) = ((p2 - p1) / 2.0).into();
                let w = OBJECT_SIZE;
                let h = p1.distance(&p2);

                let (_, mut sprite, mut transform) = query_lomid.get_mut(children[0]).unwrap();
                (transform.translation.x, transform.translation.y) = (mx, my);
                transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
                sprite.custom_size = Some(Vec2::new(w, h));

                let (ex, ey) = (p2 - p1).into();
                let (_, mut transform) = query_loend.get_mut(children[1]).unwrap();
                (transform.translation.x, transform.translation.y) = (ex, ey);
                transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
            }
            if time_now >= object.arrive_time + duration {
                if !played {
                    audio.play_with_settings(
                        sfx.justsound.clone(),
                        PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                    );
                    played = true;
                }
                commands.entity(e).despawn_recursive();
                continue;
            }
        } else {
            // passed the judgement line
            if time_now > object.arrive_time {
                if !played {
                    audio.play_with_settings(
                        sfx.justsound.clone(),
                        PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                    );
                    played = true;
                }
                commands.entity(e).despawn_recursive();
                continue;
            }
            (transform.translation.x, transform.translation.y) = 
                object.current_coord(time_now).into();
        }
    }

    // no elegant way to do it unless change overall design
    for (e, mut transform, object) in query_single.iter_mut() {
        // passed the judgement line
        if time_now > object.arrive_time {
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