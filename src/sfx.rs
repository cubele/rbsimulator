use bevy::prelude::*;
use crate::consts::*;

#[derive(Resource)]
pub struct SoundFX {
    pub justsound: Handle<AudioSource>,
}

fn setup_sfx(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sfx = SoundFX {
        justsound: asset_server.load("sounds\\just.wav"),
    };
    commands.insert_resource(sfx);
}

pub struct SFXPlugin;
impl Plugin for SFXPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_sfx);
    }
}