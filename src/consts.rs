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

pub const JUDGE_LINE_POSITION: f32 = INNER_WINDOW_Y_MIN + 100.;
pub const TOP_SLOT_START_X: f32 = INNER_WINDOW_X_MIN + 50.;
pub const TOP_SLOT_END_X: f32 = INNER_WINDOW_X_MAX - 50.;
pub const TOP_SLOT_SPACING: f32 = (TOP_SLOT_END_X - TOP_SLOT_START_X) / 2.;
pub const TOP_SLOT_Y: f32 = INNER_WINDOW_Y_MIN + 300.;

// notes
pub const BASE_SPEED: f32 = 400.;
pub const SPAWN_POSITION: f32 = INNER_WINDOW_Y_MAX + 50.;
pub const TARGET_POSITION: f32 = JUDGE_LINE_POSITION;
pub const OBJECT_Z: f32 = 100.;

pub const OBJECT_SIZE: f32 = 50.;
pub const BOTTOM_SLOT_START_X: f32 = INNER_WINDOW_X_MIN + INNER_WINDOW_WIDTH / 7. / 2.;
pub const BOTTOM_SLOT_SPACING: f32 = INNER_WINDOW_WIDTH / 7.;
pub const BOTTOM_SLOT_Y: f32 = JUDGE_LINE_POSITION;