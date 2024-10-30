use bevy::{asset::embedded_asset, prelude::*};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(
            app,
            "../../common_assets/fonts/atari-classic.ttf"
        );
        embedded_asset!(app, "images/straight.png");
        embedded_asset!(app, "images/dead_end.png");
        embedded_asset!(app, "images/elbow.png");
        embedded_asset!(app, "images/tee.png");
        embedded_asset!(app, "images/cross.png");
    }
}
