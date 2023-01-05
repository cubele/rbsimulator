use bevy::prelude::*;

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

#[derive(Component)]
struct Object;

/// Keeps track of when to Spawn a new arrow
#[derive(Resource)]
struct SpawnTimer(Timer);

/// Spawns arrows
fn spawn_objects(
    mut commands: Commands,
    materials: Res<ObjTexture>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(-400., 0., 1.));
    commands
        .spawn(SpriteBundle {
            texture: materials.red_obj.clone(),
            transform,
            ..default()
        })
        .insert(Object);
}

/// Moves the arrows forward
fn move_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Object)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_object_texture)
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_system(spawn_objects)
            .add_system(move_objects);
    }
}