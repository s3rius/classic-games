use bevy::{asset::embedded_asset, prelude::*};

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "fonts/atari-classic.ttf");
    }
}
