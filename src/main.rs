use bevy::{
    prelude::*,
};

mod notes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(notes::ObjectsPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}