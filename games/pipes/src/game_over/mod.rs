use bevy::prelude::*;

use crate::{
    globals::GameState,
    utils::systems::{despawn_entities, vert_menu_controls},
};

mod components;
mod systems;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), systems::render_screen)
            .add_systems(
                Update,
                (
                    systems::focused_button_decoration,
                    systems::on_select_item,
                    vert_menu_controls,
                )
                    .distributive_run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_entities::<components::OnGameOverScreen>,
            );
    }
}
