use bevy::prelude::*;
use bevy_kira_audio::prelude::{AudioSource, AudioControl, Audio};
use crate::consts::*;

#[derive(Resource)]
pub struct SoundFX {
    pub justsound: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct SFXPlayed(pub bool);

impl SFXPlayed {
    /// SFX is played once per game cycle
    pub fn try_play(&mut self, audio: &Res<Audio>, sfx: &Handle<AudioSource>) {
        if !self.0 {
            audio.play(sfx.clone())
            .with_volume(VOLUME_SFX.into());
            self.0 = true;
        }
    }
}

fn setup_sfx(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sfx = SoundFX {
        justsound: asset_server.load("sounds\\sys_shot_hockey_just.ogg"),
    };
    commands.insert_resource(sfx);
    let played = SFXPlayed(false);
    commands.insert_resource(played);
}

fn reset_played(mut sfxplayed: ResMut<SFXPlayed>) {
    sfxplayed.0 = false;
}

use super::labels::*;
pub struct SFXPlugin;
impl Plugin for SFXPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_sfx)
            .add_system(reset_played.after(RenderStage::MoveObj));
    }
}