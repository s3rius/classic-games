use bevy::{asset::embedded_asset, prelude::*};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "fonts/atari-classic.ttf");
    }
}
