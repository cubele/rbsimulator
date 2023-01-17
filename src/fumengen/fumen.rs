use bevy::prelude::*;
use bevy_kira_audio::prelude::{AudioSource};
use crate::objects::Object;
use crate::chains::Chain;
use crate::slider::Slider;

pub struct FumenMetadata {
    /// song name
    pub name: String,
    pub artist: String,
    pub charter: String,
    /// range
    pub bpm: String,
    pub difficulty: String,
    pub level: u32,
}

#[derive(Resource)]
pub struct Fumen {
    pub metadata: FumenMetadata,
    pub objects: Vec<Object>,
    pub current: usize,
    pub chains: Vec<Chain>,
    pub currentchain: usize,
    pub sliders: Vec<Slider>,
    pub currentslider: usize,
    pub song_audio: Handle<AudioSource>,
    pub playing: bool,
    pub song_start_time: f64,
    pub seconds_per_measure: f64,
    pub delay: f64,
    pub song_offset: f64,
    pub bpm: f64,
}

impl Fumen {
    pub fn current_object(&self) -> Option<&Object> {
        self.objects.get(self.current as usize)
    }

    pub fn goto_next_object_with_same_side(&mut self, side: u32) {
        self.current += 1;
        while let Some(object) = self.current_object() {
            if object.side == side {
                self.current += 1;
            } else {
                break;
            }
        }
    }

    pub fn current_chain(&self) -> Option<&Chain> {
        self.chains.get(self.currentchain as usize)
    }

    pub fn current_slider(&self) -> Option<&Slider> {
        self.sliders.get(self.currentslider as usize)
    }

    pub fn relative_time(&self, time: &Res<Time>) -> f64 {
        time.elapsed_seconds_f64() - self.song_start_time
    }

    /// bpm changes are not supported yet
    pub fn current_bpm(&self, _time: f64) -> f64 {
        self.bpm
    }
}