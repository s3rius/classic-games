use std::path::{Path, PathBuf};

use bevy::{asset::io::embedded::EmbeddedAssetRegistry, prelude::*};

pub const COMMON_FONT_NAME: &str = "embedded://cgc_common/atari-classic.ttf";

pub struct CGCCommonAssetsPlugin;

impl Plugin for CGCCommonAssetsPlugin {
    fn build(&self, app: &mut App) {
        let registry = app.world_mut().resource_mut::<EmbeddedAssetRegistry>();
        // Default font for all games
        registry.insert_asset(
            PathBuf::from(file!()).join("assets/fonts/atari-classic.ttf"),
            Path::new(COMMON_FONT_NAME.strip_prefix("embedded://").unwrap()),
            include_bytes!("assets/fonts/atari-classic.ttf"),
        );
    }
}
