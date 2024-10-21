use bevy::prelude::*;
use systems::{focused_button_decoration, on_select_item};

use crate::{
    state::GameState,
    utils::systems::{despawn_screen, vert_menu_controls},
};
mod components;
mod systems;

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), systems::render_screen)
            .add_systems(
                Update,
                (
                    vert_menu_controls,
                    on_select_item,
                    focused_button_decoration,
                )
                    .run_if(in_state(GameState::StartMenu)),
            )
            .add_systems(
                OnExit(GameState::StartMenu),
                despawn_screen::<components::OnMenuScreen>,
            );
    }
}
