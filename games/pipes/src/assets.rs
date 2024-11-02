use bevy::{asset::embedded_asset, prelude::*};
use cgc_lib::CGCCommonAssetsPlugin;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CGCCommonAssetsPlugin);
        embedded_asset!(app, "images/straight.png");
        embedded_asset!(app, "images/dead_end.png");
        embedded_asset!(app, "images/elbow.png");
        embedded_asset!(app, "images/tee.png");
        embedded_asset!(app, "images/cross.png");
    }
}
