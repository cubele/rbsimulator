mod game;
mod fumengen;
#[allow(non_snake_case, dead_code, unused)]
mod tests;
mod utils;

use fumengen::{parse::FumenDescription};
pub use game::*;
pub use fumengen::*;
pub use utils::*;

use bevy::prelude::*;
use game::consts::*;

fn main() {
    let defaultplugins = DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Rb poor".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            position: WindowPosition::Centered,
            resizable: false,
            ..default()
        },
        ..default()
    });
    App::new()
        .add_plugins(defaultplugins)
        .add_plugin(objects::ObjectsPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(sfx::SFXPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let testfumen = FumenDescription::from_json(
        "orbital", "あさき隊", "黑猫"
    ).unwrap()
    .into_fumen(&asset_server);
    commands.insert_resource(testfumen);
}