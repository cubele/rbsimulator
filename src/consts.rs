// UI
pub const WINDOW_HEIGHT: f32 = 1080.;
pub const WINDOW_WIDTH: f32 = 720.;
/// where the game is played
pub const INNER_WINDOW_HEIGHT: f32 = 1000.;
pub const INNER_WINDOW_WIDTH: f32 = 640.;
pub const INNER_WINDOW_X_PAD: f32 = (WINDOW_WIDTH - INNER_WINDOW_WIDTH) / 2.;
pub const INNER_WINDOW_Y_PAD: f32 = (WINDOW_HEIGHT - INNER_WINDOW_HEIGHT) / 2.;
/// real coordinates
/// 0,0 is the center of the screen
pub const INNER_WINDOW_X_MIN: f32 = -INNER_WINDOW_WIDTH / 2.;
pub const INNER_WINDOW_X_MAX: f32 = INNER_WINDOW_WIDTH / 2.;
pub const INNER_WINDOW_Y_MIN: f32 = -INNER_WINDOW_HEIGHT / 2.;
pub const INNER_WINDOW_Y_MAX: f32 = INNER_WINDOW_HEIGHT / 2.;

// notes
pub const BASE_SPEED: f32 = 200.;
pub const SPAWN_POSITION: f32 = INNER_WINDOW_Y_MAX + 50.;
pub const TARGET_POSITION: f32 = INNER_WINDOW_Y_MIN + 100.;
