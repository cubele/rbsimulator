use bevy::prelude::*;
use super::consts::*;
use super::sfx::SoundFX;
use super::objects::*;

pub fn load_object_texture(
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
        slide_up: asset_server.load("images\\redSOup.png"),
        slide_right: asset_server.load("images\\redSOright.png"),
        slide_left: asset_server.load("images\\redSOleft.png"),
        slide_mid: asset_server.load("images\\SOmiddle.png"),
        slide_end: asset_server.load("images\\redSOend.png"),
        chain: asset_server.load("images\\chain.png"),
        glow: asset_server.load("images\\glow.png"),
    });
}

#[derive(Component)]
pub struct LoMid;

#[derive(Component)]
pub struct LoEnd;

#[derive(Component)]
pub struct ChordGlow;

use crate::fumen::Fumen;
pub fn spawn_objects(
    mut commands: Commands,
    mut fumen: ResMut<Fumen>,
    materials: Res<ObjTexture>,
    time: Res<Time>,
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    while let Some(object) = fumen.current_object() {
        if object.spawn_time < time_now {
            // sliders are rendered seperately
            if object.objtype != Objecttype::Slide {
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
                        unreachable!();
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
                    let sprite = Sprite {
                        custom_size: Some(Vec2::new(OBJECT_SIZE * 1.1, OBJECT_SIZE * 1.1)),
                        ..default()
                    };
                    e.add_children(|parent| {
                        parent.spawn(SpriteBundle {
                            sprite: sprite.clone(),
                            texture: materials.glow.clone(),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                            ..default()
                        }).insert(ChordGlow);
                    });
                }
                e.insert(*object);
            }
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
pub fn move_objects(
    mut commands: Commands, time: Res<Time>,
    // these shennanigans are to prevent intersection between queries
    mut query_single: Query<(Entity, &mut Transform, &Object), Without<Children>>,
    mut query: Query<(Entity, &mut Transform, &Object, &Children)>,
    // middle part of LO, used to get the entity from child id
    mut query_lomid: Query<(Entity, &mut Sprite, &mut Transform), (With<LoMid>, Without<LoEnd>, Without<Object>)>,
    mut query_loend: Query<(Entity, &mut Sprite, &mut Transform), (With<LoEnd>, Without<LoMid>, Without<Object>)>,
    audio: Res<Audio>, sfx: Res<SoundFX>, fumen: Res<Fumen>
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    let time_last = time_now - time.delta_seconds_f64();
    // info!("delta time: {:?}", time.delta_seconds_f64());
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
    // info!("size: {:?}", query.iter().len() + query_single.iter().len());
}
