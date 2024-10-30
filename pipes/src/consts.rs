use bevy::color::Color;

pub const CELL_SIZE: f32 = 64.;

pub const BACKGROUND_COLOR: Color = Color::srgb(0., 0., 0.);
pub const FOREGROUND_COLOR: Color = Color::srgb(0., 1., 0.);

pub const PIPE_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const FOCUSED_PIPE_COLOR: Color = Color::srgb(1., 0.95, 0.607);
pub const CONNECTED_PIPE_COLOR: Color = Color::srgb(0., 0.7, 0.);
pub const CYCLE_PIPES_COLOR: Color = Color::srgb(1., 0., 0.);

pub const FONT_RESOURCE: &str = "embedded://pipes/../../common_assets/fonts/atari-classic.ttf";

pub const PIPE_CROSS_TEXTURE: &str = "embedded://pipes/images/cross.png";
pub const PIPE_DEAD_END_TEXTURE: &str = "embedded://pipes/images/dead_end.png";
pub const PIPE_ELBOW_TEXTURE: &str = "embedded://pipes/images/elbow.png";
pub const PIPE_STRAIGH_TEXTURE: &str = "embedded://pipes/images/straight.png";
pub const PIPE_TEE_TEXTURE: &str = "embedded://pipes/images/tee.png";
