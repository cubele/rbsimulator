pub mod game;
pub mod fumengen;
#[allow(non_snake_case, dead_code, unused)]
pub mod tests;
pub mod utils;
pub mod cli;

use std::path::Path;

pub use game::*;
pub use fumengen::*;
pub use utils::*;

use bevy_kira_audio::prelude::*;
use bevy::prelude::*;
use game::consts::*;
use clap::Parser;

use crate::{parse::FumenDescription, jsonparse::JsonFumen};

fn main() {
    let defaultplugins = DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Rb poor by A79".to_string(),
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
        .add_plugin(AudioPlugin)
        .add_plugin(objects::ObjectsPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(sfx::SFXPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let args = cli::Cli::parse();
    info!("args: {:?}", args);
    let json = args.fumenpath.to_str().unwrap().ends_with(".json");
    let song_offset = args.starttime.unwrap_or(0);
    let mut fumendesc = if json {
        FumenDescription::from_json_file("unknown", "unknown", "unknown",
            args.delay.unwrap_or(0), args.fumenpath.to_str().unwrap()
        )
    } else {
        FumenDescription::from_json("unknown", "unknown", "unknown",
            args.delay.unwrap_or(0),
            JsonFumen::from_ply(args.fumenpath.to_str().unwrap())
        )
    }.unwrap();
    
    let songpath = Path::new("../").join(args.songpath);
    let fumen = fumendesc.into_fumen(
        songpath.to_str().unwrap(), 
        song_offset as f64 / 1000.0,
        &asset_server);
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(fumen);
}