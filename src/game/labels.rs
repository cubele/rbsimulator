use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum RenderStage {
    SpawnObj,
    MoveObj,
    CleanUp,
}