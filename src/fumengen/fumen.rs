use bevy::prelude::*;
use crate::objects::Object;
use crate::chains::Chain;

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
    pub chains: Vec<Chain>,
    pub current: usize,
    pub currentchain: usize,
    pub song_audio: Handle<AudioSource>,
    pub playing: bool,
    pub song_start_time: f64,
    pub seconds_per_measure: f64,
    pub delay: f64,
}

impl Fumen {
    pub fn current_object(&self) -> Option<&Object> {
        self.objects.get(self.current as usize)
    }

    pub fn current_chain(&self) -> Option<&Chain> {
        self.chains.get(self.currentchain as usize)
    }
}