use bevy::{
    prelude::*,
};

mod objects;
mod consts;
mod fumen;
mod ui;
mod utils;
mod coords;
mod sfx;
mod audio;

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
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let testfumen = fumen::Fumen::dummy(&asset_server);
    commands.insert_resource(testfumen);
}