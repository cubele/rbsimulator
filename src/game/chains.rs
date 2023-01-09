use super::objects::*;
use bevy::prelude::*;
use crate::fumen::Fumen;
use super::consts::*;

#[derive(Component, Clone, Copy, Debug)]
/// Objects in the chain should have the same spawn coord and dest coord
/// This is only used to render chains, the Objects are rendered separately
pub struct Chain {
    /// chained first to last
    pub head: Object,
    pub tail: Object,
}

impl Chain {
    pub fn spawn_time(&self) -> f64 {
        self.head.spawn_time
    }
}

/// chains aren't implemented as child of notes because they need information of the next node
pub fn spawn_chains(
    mut commands: Commands,
    mut fumen: ResMut<Fumen>,
    materials: Res<ObjTexture>,
    time: Res<Time>,
) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    while let Some(chain) = fumen.current_chain() {
        if chain.spawn_time() < time_now {
            let (head, tail) = (chain.head, chain.tail);
            let p1 = head.current_coord(time_now);
            let p2 = tail.current_coord(time_now);
            // expand the line to a rectangle
            let w = CHAIN_WIDTH;
            let h = p1.distance(&p2);
            let angle = p1.angle(&p2);
            let (mx, my) = ((p1 + p2) / 2.0).into();
            let transform = Transform::from_xyz(
                    mx, my, CHAIN_Z + fumen.currentchain as f32 * CHAIN_Z_DIFF)
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle));
            commands.spawn(SpriteBundle {
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                    ..default()
                },
                texture: materials.chain.clone(),
                ..default()
            }).insert(*chain);
            fumen.currentchain += 1;
        } else {
            break;
        }
    }
}

pub fn move_chains(mut commands: Commands, time: Res<Time>,
               mut query: Query<(Entity, &mut Transform, &mut Sprite, &Chain)>,
               fumen: Res<Fumen>) {
    let time_now = time.elapsed_seconds_f64() - fumen.song_start_time;
    for (e, mut transform, mut sprite, chain) in query.iter_mut() {
        let (head, tail) = (chain.head, chain.tail);
        let p1 = head.current_coord(time_now);
        let p2 = tail.current_coord(time_now);
        if p1.y() < head.dest.y() {
            commands.entity(e).despawn();
            continue;
        }
        // expand the line to a rectangle
        let w = CHAIN_WIDTH;
        let h = p1.distance(&p2);
        let angle = p1.angle(&p2);
        let (mx, my) = ((p1 + p2) / 2.0).into();
        (transform.translation.x, transform.translation.y) = (mx, my);
        transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2 + angle);
        sprite.custom_size = Some(Vec2::new(w, h));
    }
}