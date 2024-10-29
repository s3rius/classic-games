use bevy::prelude::*;
use systems::{focused_button_decoration, on_select_item};

use crate::{
    globals::GameState,
    utils::systems::{despawn_entities, vert_menu_controls},
};
mod components;
mod systems;

pub struct StartMenu;

impl Plugin for StartMenu {
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
                despawn_entities::<components::OnMenuScreen>,
            );
    }
}
