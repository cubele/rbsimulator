use bevy::prelude::*;
use crate::fumen::Fumen;
use super::consts::*;

// TODO: try starting from given time
// TODO: make this only execute once
fn start_song(audio: Res<Audio>, time: Res<Time>, mut fumen: ResMut<Fumen>) {
    let time_now = time.elapsed_seconds_f64();
    if !fumen.playing && time_now > AUDIO_DELAY {
        audio.play_with_settings(
            fumen.song_audio.clone(),
            PlaybackSettings::ONCE.with_volume(VOLUME_SONG),
        );
        fumen.playing = true;
        fumen.song_start_time = time_now;
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_song);
    }
}