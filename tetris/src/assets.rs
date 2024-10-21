use bevy::{asset::embedded_asset, prelude::*};

pub struct TetrisAssetsPlugin;

impl Plugin for TetrisAssetsPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "fonts/atari-classic.ttf");
    }
}
