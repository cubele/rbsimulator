use bevy::prelude::*;
use bevy_kira_audio::prelude::{AudioControl, Audio};
use crate::fumen::Fumen;
use super::consts::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, mut fumen: ResMut<Fumen>) {
    let time_now = time.elapsed_seconds_f64();
    if !fumen.playing && time_now > AUDIO_DELAY {
        audio.play(fumen.song_audio.clone())
        .start_from(fumen.song_offset)
        .with_volume(VOLUME_SONG.into());
        fumen.playing = true;
        fumen.song_start_time = time_now - fumen.song_offset;
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_song);
    }
}