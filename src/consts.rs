// UI
pub const RATIO: f32 = 720.0 / 1080.0;
pub const WINDOW_HEIGHT: f32 = 1080.;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * RATIO;
/// where the game is played
pub const INNER_WINDOW_HEIGHT: f32 = 1000.;
pub const INNER_WINDOW_WIDTH: f32 = 640.;
/// real coordinates
/// 0,0 is the center of the screen
pub const INNER_WINDOW_X_MIN: f32 = -INNER_WINDOW_WIDTH / 2.;
pub const INNER_WINDOW_X_MAX: f32 = INNER_WINDOW_X_MIN + INNER_WINDOW_WIDTH;
pub const INNER_WINDOW_Y_MIN: f32 = -INNER_WINDOW_HEIGHT / 2.;
pub const INNER_WINDOW_Y_MAX: f32 = INNER_WINDOW_Y_MIN + INNER_WINDOW_HEIGHT;

pub const JUDGE_LINE_POSITION: f32 = INNER_WINDOW_Y_MIN + 100.;

pub const TOP_SLOT_COUNT: u32 = 3;
pub const TOP_SLOT_COUNT_F: f32 = TOP_SLOT_COUNT as f32;
pub const TOP_SLOT_SIZE: f32 = 50.;
pub const TOP_SLOT_PADDING: f32 = 80.;
pub const TOP_SLOT_START_X: f32 = INNER_WINDOW_X_MIN + TOP_SLOT_SIZE / 2. + TOP_SLOT_PADDING;
pub const TOP_SLOT_END_X: f32 = INNER_WINDOW_X_MAX - TOP_SLOT_SIZE / 2. - TOP_SLOT_PADDING;
pub const TOP_SLOT_SPACING: f32 = (TOP_SLOT_END_X - TOP_SLOT_START_X) / (TOP_SLOT_COUNT_F - 1.0);
pub const TOP_SLOT_Y: f32 = INNER_WINDOW_Y_MIN + 300.;

// notes
pub const OBJ_TIME: f64 = 0.8;
pub const SPAWN_POSITION: f32 = INNER_WINDOW_Y_MAX + 50.;
pub const OBJECT_Z: f32 = 100.;
pub const OBJECT_Z_DIFF: f32 = 1.0 / 100000.0;

pub const TOP_SPAWN_PADDING: f32 = 200.;
pub const TOP_SPAWN_X_START: f32 = INNER_WINDOW_X_MIN + TOP_SPAWN_PADDING;
pub const TOP_SPAWN_X_END: f32 = INNER_WINDOW_X_MAX - TOP_SPAWN_PADDING;
pub const TOP_SPAWN_X_SPACING: f32 = (TOP_SPAWN_X_END - TOP_SPAWN_X_START) / 2.;

pub const BOTTOM_SLOT_COUNT: u32 = 7;
pub const BOTTOM_SLOT_COUNT_F: f32 = BOTTOM_SLOT_COUNT as f32;

pub const OBJECT_SIZE: f32 = 50.;
pub const BOTTOM_SLOT_START_X: f32 = INNER_WINDOW_X_MIN + OBJECT_SIZE / 2.;
pub const BOTTOM_SLOT_SPACING: f32 = INNER_WINDOW_WIDTH / BOTTOM_SLOT_COUNT_F;
pub const BOTTOM_SLOT_Y: f32 = JUDGE_LINE_POSITION;

pub const CHAIN_WIDTH: f32 = 10.;
pub const CHAIN_Z: f32 = 99.;
pub const CHAIN_Z_DIFF: f32 = 1.0 / 100000.0;

// audio
pub const AUDIO_DELAY: f64 = 0.1;
pub const VOLUME_SONG: f32 = 0.12;
pub const VOLUME_SFX: f32 = 0.10;