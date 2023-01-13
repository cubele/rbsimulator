use bevy::prelude::*;
use super::consts::*;
use super::coords::Coord2d; 

/// Keeps the textures and materials
#[derive(Resource)]
pub struct ObjTexture {
    pub red_obj: Handle<Image>,
    pub red_lo_start: Handle<Image>,
    pub red_lo_mid: Handle<Image>,
    pub red_lo_end: Handle<Image>,
    pub top_obj: Handle<Image>,
    pub top_lo_start: Handle<Image>,
    pub top_lo_mid: Handle<Image>,
    pub top_lo_end: Handle<Image>,
    pub vertical_obj: Handle<Image>,
    pub vertical_lo_start: Handle<Image>,
    pub slide_up: Handle<Image>,
    pub slide_right: Handle<Image>,
    pub slide_left: Handle<Image>,
    pub slide_mid: Handle<Image>,
    pub slide_end: Handle<Image>,
    pub chain: Handle<Image>,
    pub glow: Handle<Image>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Objecttype {
    Normal,
    Vertical,
    Top,
    Slide,
}

#[derive(Clone, Copy, Debug)]
pub enum SlideDirection {
    Up,
    Right,
    Left,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Object {
    pub spawn_time: f64,
    pub arrive_time: f64,
    pub reflec_time: f64,
    pub spawn: Coord2d,
    pub dest: Coord2d,
    /// if not None, travels spawn->reflect->dest
    /// for simplicy, objects that dosen't reflect have reflect = spawn
    pub reflect: Option<Coord2d>,
    pub objtype: Objecttype,
    /// for long notes
    pub duration: Option<f64>,
    pub chord: bool,
    /// 0 for red, 1 for blue
    pub side: u32,
}

impl Object {
    pub fn destination(objtype: Objecttype, pos: u32, side: u32) -> Coord2d {
        let x = match objtype {
            Objecttype::Top => TOP_SLOT_START_X + TOP_SLOT_SPACING * pos as f32,
            _ => BOTTOM_SLOT_START_X + BOTTOM_SLOT_SPACING * pos as f32,
        };
        let y = match objtype {
            Objecttype::Top => {
                if side == 1 {
                    TOP_SLOT_OPPONENT_Y
                } else {
                    TOP_SLOT_Y
                }
            }
            _ => {
                if side == 1 {
                    BOTTOM_SLOT_OPPONENT_Y
                } else {
                    BOTTOM_SLOT_Y
                }
            }
        };
        Coord2d::new(x, y)
    }

    pub fn reflec_time(&self) -> f64 {
        self.reflec_time
    }

    pub fn reflect_stage(&self, time: f64) -> Option<u32> {
        if time < self.reflec_time() {
            Some(0)
        } else {
            Some(1)
        }
    }

    pub fn current_coord(&self, time: f64) -> Coord2d {
        if let Some(reflec) = self.reflect {
            let rtime = self.reflec_time();
            if time < rtime {
                let t = (time - self.spawn_time) / (rtime - self.spawn_time);
                self.spawn + (reflec - self.spawn) * t as f32
            } else {
                let t = (time - rtime) / (self.arrive_time - rtime);
                reflec + (self.dest - reflec) * t as f32
            }
        } else {
            let t = (time - self.spawn_time) / (self.arrive_time - self.spawn_time);
            self.spawn + (self.dest - self.spawn) * t as f32
        }
    }

    pub fn new(spawn_time: f64, arrive_time: f64, side: u32,
               spawn_x: f32, spawn_y: f32, objtype: Objecttype, pos: u32,
               reflect: Option<(f32, f32)>, duration: Option<f64>, chord: bool) -> Self {
        let spawn = (spawn_x, spawn_y).into();
        let dest = Self::destination(objtype, pos, side);
        let reflect : Option<Coord2d>= reflect.map(|(x, y)| (x, y).into());
        let reflec_time = if let Some(reflec) = reflect {
            let d1 = reflec.distance(&spawn);
            let d2 = reflec.distance(&dest);
            (d1 / (d1 + d2)) as f64 * (arrive_time - spawn_time) + spawn_time
        } else {
            0.
        };
        let object = Self {
            spawn_time, arrive_time, side,
            reflec_time,
            spawn,
            objtype,
            dest,
            reflect,
            duration,
            chord,
        };
        object
    }
}

use super::chains::*;
use super::render::*;
use super::slider::*;
use super::labels::*;
pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_object_texture)
            .add_system_set(
                SystemSet::new()
                    .label(RenderStage::SpawnObj)
                    .with_system(spawn_objects)
                    .with_system(spawn_chains)
                    .with_system(spawn_sliders)
            )
            .add_system_set(
                SystemSet::new()
                    .label(RenderStage::MoveObj)
                    .after(RenderStage::SpawnObj)
                    .with_system(move_objects)
                    .with_system(move_chains)
                    .with_system(move_sliders)
            );
    }
}