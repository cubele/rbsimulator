use bevy::prelude::*;
use super::consts::*;
use super::coords::Coord2d;
use super::sfx::SoundFX;

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
    Slide,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Object {
    pub spawn_time: f64,
    pub arrive_time: f64,
    pub spawn: Coord2d,
    pub dest: Coord2d,
    /// if not None, travels spawn->reflect->dest
    /// for simplicy, objects that dosen't reflect have reflect = spawn
    pub reflect: Option<Coord2d>,
    pub objtype: Objecttype,
    /// for long notes
    pub duration: Option<f64>,
    pub chord: bool,
    /// 0 for red, 1 for blue
    pub side: u32,
}

impl Object {
    pub fn destination(objtype: Objecttype, pos: u32, side: u32) -> Coord2d {
        let x = match objtype {
            Objecttype::Top => TOP_SLOT_START_X + TOP_SLOT_SPACING * pos as f32,
            _ => BOTTOM_SLOT_START_X + BOTTOM_SLOT_SPACING * pos as f32,
        };
        let y = match objtype {
            Objecttype::Top => {
                if side == 1 {
                    TOP_SLOT_OPPONENT_Y
                } else {
                    TOP_SLOT_Y
                }
            }
            _ => {
                if side == 1 {
                    BOTTOM_SLOT_OPPONENT_Y
                } else {
                    BOTTOM_SLOT_Y
                }
            }
        };
        Coord2d::new(x, y)
    }

    pub fn reflec_time(&self) -> f64 {
        if let Some(reflec) = self.reflect {
            let d1 = reflec.distance(&self.spawn);
            let d2 = reflec.distance(&self.dest);
            (d1 / (d1 + d2)) as f64 * (self.arrive_time - self.spawn_time) + self.spawn_time
        } else {
            0.
        }
    }

    pub fn reflect_stage(&self, time: f64) -> Option<u32> {
        if self.reflect.is_some() {
            let rtime = self.reflec_time();
            if time < rtime {
                Some(0)
            } else {
                Some(1)
            }
        } else {
            Some(1)
        }
    }

    pub fn current_coord(&self, time: f64) -> Coord2d {
        if let Some(reflec) = self.reflect {
            let rtime = self.reflec_time();
            if time < rtime {
                let t = (time - self.spawn_time) / (rtime - self.spawn_time);
                self.spawn + (reflec - self.spawn) * t as f32
            } else {
                let t = (time - rtime) / (self.arrive_time - rtime);
                reflec + (self.dest - reflec) * t as f32
            }
        } else {
            let t = (time - self.spawn_time) / (self.arrive_time - self.spawn_time);
            self.spawn + (self.dest - self.spawn) * t as f32
        }
    }

    pub fn new(spawn_time: f64, arrive_time: f64, side: u32,
               spawn_x: f32, spawn_y: f32, objtype: Objecttype, pos: u32,
               reflect: Option<(f32, f32)>, duration: Option<f64>, chord: bool) -> Self {
        let object = Self {
            spawn_time, arrive_time, side,
            spawn: (spawn_x, spawn_y).into(),
            objtype,
            dest: Self::destination(objtype, pos, side),
            reflect: reflect.map(|(x, y)| (x, y).into()),
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
                Objecttype::Slide => {
                    unimplemented!();
                }
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
            // spawns LO child node
            if object.duration.is_some() {
                let (midtexture, endtexture) = match object.objtype {
                    Objecttype::Top => (materials.top_lo_mid.clone(), materials.top_lo_end.clone()),
                    Objecttype::Vertical => (materials.red_lo_mid.clone(), materials.red_lo_end.clone()),
                    Objecttype::Normal => (materials.red_lo_mid.clone(), materials.red_lo_end.clone()),
                    Objecttype::Slide => {
                        unimplemented!();
                    }
                };
                let transform_mid = Transform::from_xyz(0., 0., -0.5);
                let transform_end = Transform::from_xyz(0., 0., -1.);
                
                e.add_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(0., 0.)),
                            ..default()
                        },
                        texture: midtexture,
                        transform: transform_mid,
                        ..default()
                    }).insert(LoMid);
                });
                
                e.add_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(0., 0.)),
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
            while let Some(object) = fumen.current_object() {
                if object.side == 1 {
                    fumen.current += 1;
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
}

/// also plays sfx for lower latency
fn move_objects(
    mut commands: Commands, time: Res<Time>,
    // these shennanigans are to prevent intersection between queries
    mut query_single: Query<(Entity, &mut Transform, &Object), (Without<LoMid>, Without<LoMid>, Without<Children>)>,
    mut query: Query<(Entity, &mut Transform, &Object, &Children), (Without<LoMid>, Without<LoMid>)>,
    // middle part of LO, used to get the entity from child id
    mut query_lomid: Query<(Entity, &mut Sprite, &mut Transform), (With<LoMid>, Without<LoEnd>, Without<Object>)>,
    mut query_loend: Query<(Entity, &mut Sprite, &mut Transform), (With<LoEnd>, Without<LoMid>, Without<Object>)>,
    audio: Res<Audio>, sfx: Res<SoundFX>, fumen: Res<Fumen>
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    let time_last = time_now - time.delta_seconds_f64();
    let mut played = false;
    for (e,
        mut transform,
        object,
        children) in query.iter_mut() {
        // render LO
        if let Some(duration) = object.duration {
            let disp_duration = duration.min(LO_DISP_TIME_MAX);
            // hasn't arrived
            if time_now < object.arrive_time {
                (transform.translation.x, transform.translation.y) = 
                    object.current_coord(time_now).into();
                // long objects dosen't extend before reflect
                // This should always be the case for now since normal objects also have reflec
                if let Some(stage) = object.reflect_stage(time_now) {
                    // where LOs should start to extend
                    if stage > 0 {
                        let time_end = (time_now - duration.min(LO_DISP_TIME_MAX))
                            .max(object.reflec_time());
                        if time_end < time_now {
                            let p1 = object.current_coord(time_now);
                            let p2 = object.current_coord(time_end);
                            // expand the line to a rectangle
                            let w = OBJECT_SIZE;
                            let h = p1.distance(&p2);
                            let angle = p1.angle(&p2);
                            let (mx, my) = ((p2 - p1) / 2.0).into();
                            let (_, mut sprite, mut transform) = query_lomid.get_mut(children[0]).unwrap();
                            (transform.translation.x, transform.translation.y) = (mx, my);
                            transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
                            sprite.custom_size = Some(Vec2::new(w, h));

                            let (ex, ey) = (p2 - p1).into();
                            let (_, mut sprite, mut transform) = query_loend.get_mut(children[1]).unwrap();
                            (transform.translation.x, transform.translation.y) = (ex, ey);
                            transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
                            sprite.custom_size = Some(Vec2::new(OBJECT_SIZE, OBJECT_SIZE));
                        }
                    }
                }
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

                //let angle = p1.angle(&p2);
                let (mx, my) = ((p2 - p1) / 2.0).into();
                let w = OBJECT_SIZE;
                let h = p1.distance(&p2);

                let (_, mut sprite, mut transform) = query_lomid.get_mut(children[0]).unwrap();
                (transform.translation.x, transform.translation.y) = (mx, my);
                //transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
                sprite.custom_size = Some(Vec2::new(w, h));

                let (ex, ey) = (p2 - p1).into();
                let (_, _, mut transform) = query_loend.get_mut(children[1]).unwrap();
                (transform.translation.x, transform.translation.y) = (ex, ey);
                //transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
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
            }
        } else {
            // passed the judgement line
            if time_now >= object.arrive_time {
                if !played {
                    audio.play_with_settings(
                        sfx.justsound.clone(),
                        PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                    );
                    played = true;
                }
                commands.entity(e).despawn_recursive();
            } else {
                (transform.translation.x, transform.translation.y) = 
                    object.current_coord(time_now).into();
            }
        }
    }

    // no elegant way to do it unless change overall design
    for (e, mut transform, object) in query_single.iter_mut() {
        // passed the judgement line
        if time_now >= object.arrive_time {
            if !played {
                audio.play_with_settings(
                    sfx.justsound.clone(),
                    PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                );
                played = true;
            }
            commands.entity(e).despawn_recursive();
        } else {
            (transform.translation.x, transform.translation.y) = 
                object.current_coord(time_now).into();
        }
    }
}

use super::chains::*;
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