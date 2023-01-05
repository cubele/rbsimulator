use bevy::{
    prelude::*,
};

mod notes;
mod consts;
mod types;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(notes::ObjectsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let testfumen = types::Fumen::dummy();
    commands.insert_resource(testfumen);
}