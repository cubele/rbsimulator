use bevy::prelude::*;
use crate::consts::*;

/// Keeps the textures and materials
#[derive(Resource)]
struct ObjTexture {
    red_obj: Handle<Image>,
}

fn load_object_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ObjTexture{
        red_obj: asset_server.load("images\\redobject.png"),
    });
}

use crate::types::{Object, Fumen};

fn spawn_objects(
    mut commands: Commands,
    mut fumen: ResMut<Fumen>,
    materials: Res<ObjTexture>,
    time: Res<Time>,
) {
    let secs = time.elapsed_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for object in &fumen.objects {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if object.spawn_time < secs {
            remove_counter += 1;

            let transform =
                Transform::from_translation(Vec3::new(0., SPAWN_POSITION, 1.));
            commands.spawn(SpriteBundle {
                texture: materials.red_obj.clone(),
                transform,
                ..default()
            })
            .insert(*object);
        } else {
            break;
        }
    }

    for _ in 0..remove_counter {
        fumen.objects.remove(0);
    }
}

fn move_objects(time: Res<Time>, mut query: Query<(&mut Transform, &mut Visibility, &Object)>) {
    for (mut transform, mut visibility, _object) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * BASE_SPEED;
        // passed the judgement line
        if transform.translation.y < TARGET_POSITION {
            visibility.is_visible = false;
        }
    }
}

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_object_texture)
            .add_system(spawn_objects)
            .add_system(move_objects);
    }
}