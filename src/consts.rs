/// Speed at which a Slow arrow moves
pub const BASE_SPEED: f32 = 200.;

/// Y coordinate value at which arrows spawn, should be out of screen
pub const SPAWN_POSITION: f32 = 400.;

/// Y coordinate value where the arrows should be clicked
pub const TARGET_POSITION: f32 = 200.;

/// Margin of error for clicking an arrow
pub const THRESHOLD: f32 = 20.;

/// Total distance traveled by an arrow, from spawn to target
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;