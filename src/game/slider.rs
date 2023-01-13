use super::objects::*;
use bevy::prelude::*;
use crate::fumen::Fumen;
use super::consts::*;
use super::coords::*;
use crate::sfx::*;

#[derive(Clone, Copy, Debug)]
pub struct SlidePoint {
    pub pos: u32,
    pub spawn_time: f64,
    pub arrive_time: f64,
    pub spawn: Coord2d,
    pub dest: Coord2d,
}

impl SlidePoint {
    pub fn current_coord(&self, time: f64) -> Coord2d {
        let t = (time - self.spawn_time) / (self.arrive_time - self.spawn_time);
        self.spawn + (self.dest - self.spawn) * t as f32
    }
}

#[derive(Component, Clone, Debug)]
/// The start of slider is rendered as a normal VO
/// the rest is rendered here
pub struct Slider {
    pub parts: Vec<SlidePoint>,
}

impl Slider {
    pub fn spawn_time(&self) -> f64 {
        self.parts[0].spawn_time
    }
}

#[derive(Component, Clone, Debug)]
pub struct SliderNode;

#[derive(Component, Clone, Debug)]
pub struct SliderSegment;

pub fn spawn_sliders(
    mut commands: Commands,
    mut fumen: ResMut<Fumen>,
    materials: Res<ObjTexture>,
    time: Res<Time>,
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    while let Some(slider) = fumen.current_slider() {
        if slider.spawn_time() < time_now {
            let head = slider.parts[0];
            let headcoord= head.current_coord(time_now);
            let (mx, my) = headcoord.into();
            let transform = Transform::from_xyz(
                    mx, my, SLIDE_Z + fumen.currentslider as f32 * SLIDE_Z_DIFF);
            let next = slider.parts[1];
            commands.spawn(SpriteBundle {
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)),
                    ..default()
                },
                texture: if head.pos == next.pos {
                    materials.slide_up.clone()
                } else if head.pos < next.pos {
                    materials.slide_right.clone()
                } else {
                    materials.slide_left.clone()
                },
                ..default()
            }).with_children(|parent| {
                let mut final_angle = 0.;
                let mut directions = vec![];
                // segments
                for (id, (start, end)) in slider.parts.iter().zip(slider.parts.iter().skip(1)).enumerate() {
                    let p1 = start.current_coord(time_now);
                    let p2 = end.current_coord(time_now);
                    let w = OBJECT_SIZE;
                    let angle = p1.angle(&p2);
                    final_angle = angle;
                    let (mx, my) = ((p1 + p2) / 2.0 - headcoord).into();
                    let transform = Transform::from_xyz(
                            mx, my, -1. + CHAIN_Z_DIFF * id as f32)
                            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle));
                    parent.spawn(SpriteBundle {
                        transform,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(w, 0.)),
                            color: Color::rgba(1., 1., 1., 0.7),
                            ..default()
                        },
                        texture: materials.slide_mid.clone(),
                        ..default()
                    }).insert(SliderSegment);
                    if start.pos == end.pos {
                        directions.push(SlideDirection::Up);
                    } else if start.pos < end.pos {
                        directions.push(SlideDirection::Right);
                    } else {
                        directions.push(SlideDirection::Left);
                    }
                }
                // nodes
                for (i, point) in slider.parts.iter().enumerate() {
                    if i == 0 {
                        continue;
                    }
                    let coord = point.current_coord(time_now) - headcoord;
                    let (mx, my) = coord.into();
                    let mut transform = Transform::from_xyz(
                            mx, my, i as f32);
                    if i == slider.parts.len() - 1 {
                        transform.rotate(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + final_angle));
                    }
                    parent.spawn(SpriteBundle {
                        transform,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)),
                            ..default()
                        },
                        visibility: Visibility::INVISIBLE,
                        texture: if let Some(direction) = directions.get(i) {
                            match direction {
                                SlideDirection::Up => materials.slide_up.clone(),
                                SlideDirection::Right => materials.slide_right.clone(),
                                SlideDirection::Left => materials.slide_left.clone(),
                            }
                        } else {
                            materials.slide_end.clone()
                        },
                        ..default()
                    }).insert(SliderNode);
                }
            }).insert(slider.clone());
            fumen.currentslider += 1;
        } else {
            break;
        }
    }
}

pub fn move_sliders(mut commands: Commands, time: Res<Time>,
               mut query: Query<(Entity, &mut Transform, &mut Sprite, &Children, &Slider), (Without<SliderNode>, Without<SliderSegment>)>,
               mut query_seg: Query<(&mut Transform, &mut Sprite), (Without<Children>, With<SliderSegment>, Without<SliderNode>)>,
               mut query_node: Query<(&mut Transform, &mut Visibility), (Without<Children>, Without<SliderSegment>, With<SliderNode>)>,
               fumen: Res<Fumen>, audio: Res<Audio>, sfx: Res<SoundFX>, mut played: ResMut<SFXPlayed>) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    let time_last = time_now - time.delta_seconds_f64();
    for (e,
        mut transform,
        mut sprite,
        children,
        slider) in query.iter_mut() {
        let end_time = slider.parts.last().unwrap().arrive_time;
        if time_now >= end_time {
            if !played.0 {
                audio.play_with_settings(
                    sfx.justsound.clone(),
                    PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                );
                played.0 = true;
            }
            commands.entity(e).despawn_recursive();
            continue;
        }
        
        let head = slider.parts[0];
        let mut headcoord= head.current_coord(time_now);
        let next = slider.parts[1];
        if time_now < head.arrive_time {
            (transform.translation.x, transform.translation.y) = headcoord.into();
        }
        if time_now >= head.arrive_time && time_last < head.arrive_time {
            if !played.0 {
                audio.play_with_settings(
                    sfx.justsound.clone(),
                    PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                );
                played.0 = true;
            }
        }
        // make the first object invisible
        if time_now > next.arrive_time {
            sprite.custom_size = Some(Vec2::new(0., 0.));
        }

        let segcount = slider.parts.len() - 1;
        // segments 
        for (id, (start, end)) 
        in slider.parts.iter().zip(slider.parts.iter().skip(1)).enumerate() {
            let mut p1 = start.current_coord(time_now);
            let mut p2 = end.current_coord(time_now);
            // SO starts to show up here
            if p1.y() > SLIDE_GEN_Y {
                continue;
            }
            // cut off
            if p2.y() > SLIDE_GEN_Y {
                if let Some(slope) = p1.slope(&p2) {
                    let y = SLIDE_GEN_Y;
                    let x = (y - p1.y()) / slope + p1.x();
                    p2 = (x, y).into();
                } else {
                    p2 = (p2.x(), SLIDE_GEN_Y).into();
                }
            }
            if p1.y() < JUDGE_LINE_POSITION {
                if let Some(slope) = p1.slope(&p2) {
                    let y = JUDGE_LINE_POSITION;
                    let x = (y - p1.y()) / slope + p1.x();
                    p1 = (x, y).into();
                } else {
                    p1 = (p1.x(), JUDGE_LINE_POSITION).into();
                }
                // last node
                if id == 0 {
                    (transform.translation.x, transform.translation.y) = p1.into();
                    headcoord = p1;
                } else {
                    let (mut transform, vis) = query_node.get_mut(children[id + segcount - 1]).unwrap();
                    if vis.is_visible {
                        (transform.translation.x, transform.translation.y) = (p1 - headcoord).into();
                    }
                }
            }

            let w = OBJECT_SIZE;
            let h = p1.distance(&p2);
            let angle = p1.angle(&p2);
            let (mx, my) = (((p1 + p2) / 2.0) - headcoord).into();
            // end of segment
            if time_now < end.arrive_time {
                let (mut transform, mut sprite) = query_seg.get_mut(children[id]).unwrap();
                (transform.translation.x, transform.translation.y) = (mx, my);
                transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
                sprite.custom_size = Some(Vec2::new(w, h));
                let (mut transform, mut vis) = query_node.get_mut(children[id + segcount]).unwrap();
                (transform.translation.x, transform.translation.y) = (p2 - headcoord).into();
                *vis = Visibility::VISIBLE;
            } else {
                if time_last < end.arrive_time {
                    if !played.0 {
                        audio.play_with_settings(
                            sfx.justsound.clone(),
                            PlaybackSettings::ONCE.with_volume(VOLUME_SFX),
                        );
                        played.0 = true;
                    }
                }
                let (mut transform, mut vis) = query_node.get_mut(children[id + segcount]).unwrap();
                // next of p2
                if let Some(next) = slider.parts.get(id + 2) {
                    if time_now < next.arrive_time {
                        (transform.translation.x, transform.translation.y) = (p2 - headcoord).into();
                        *vis = Visibility::VISIBLE;
                    } else {
                        *vis = Visibility::INVISIBLE;
                    }
                } else {
                    // end, will despawn naturally
                    (transform.translation.x, transform.translation.y) = (p2 - headcoord).into();
                }
                let (_, mut sprite) = query_seg.get_mut(children[id]).unwrap();
                sprite.custom_size = Some(Vec2::new(0., 0.));
            }
        }
    }
}