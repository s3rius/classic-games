use bevy::prelude::*;
use systems::{focused_button_decoration, on_select_item};

use crate::{
    state::GameState,
    utils::systems::{despawn_screen, vert_menu_controls},
};
mod components;
mod systems;

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), systems::render_screen)
            .add_systems(
                Update,
                (
                    vert_menu_controls,
                    on_select_item,
                    focused_button_decoration,
                )
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_screen::<components::OnDeathScreen>,
            );
    }
}
