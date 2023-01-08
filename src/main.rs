use bevy::prelude::*;

mod objects;
mod chains;
#[allow(dead_code)]
mod consts;
mod fumen;
mod ui;
mod utils;
mod coords;
mod sfx;
mod audio;
mod parse;
mod tests;

use consts::*;

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
    let testfumen = tests::INORI(&asset_server);
    commands.insert_resource(testfumen);
}