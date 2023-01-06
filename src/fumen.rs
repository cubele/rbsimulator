use bevy::prelude::*;
use crate::objects::Object;
use crate::chains::Chain;

#[derive(Resource, Debug)]
pub struct Fumen {
    pub objects: Vec<Object>,
    pub chains: Vec<Chain>,
    pub current: usize,
    pub currentchain: usize,
    pub song_audio: Handle<AudioSource>,
    pub playing: bool,
    pub song_start_time: f64,
}

impl Fumen {
    pub fn current_object(&self) -> Option<&Object> {
        self.objects.get(self.current as usize)
    }

    pub fn current_chain(&self) -> Option<&Chain> {
        self.chains.get(self.currentchain as usize)
    }
}