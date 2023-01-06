use bevy::{
    prelude::*,
};

mod objects;
mod consts;
mod fumen;
mod ui;
mod utils;
mod coords;

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
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let testfumen = fumen::Fumen::dummy();
    commands.insert_resource(testfumen);
}