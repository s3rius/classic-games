use bevy::prelude::*;
use cgc_lib::CGCCommonAssetsPlugin;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CGCCommonAssetsPlugin);
    }
}
