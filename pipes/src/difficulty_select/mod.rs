use bevy::prelude::*;

use crate::{
    globals::GameState,
    utils::systems::{despawn_entities, focus_first, vert_menu_controls},
};

mod components;
mod states;
mod systems;

pub struct DifficultySelect;

impl Plugin for DifficultySelect {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<states::DifficultySelectState>()
            .add_systems(OnEnter(GameState::SelectDifficulty), systems::init_substate)
            .add_systems(
                OnEnter(states::DifficultySelectState::SelectingSize),
                (systems::setup_size_select, focus_first).chain(),
            )
            .add_systems(
                OnEnter(states::DifficultySelectState::SelectingWrap),
                (systems::setup_wrap_select, focus_first).chain(),
            )
            .add_systems(
                Update,
                (
                    vert_menu_controls,
                    systems::focused_button_decoration,
                    systems::handle_select_size
                        .run_if(in_state(states::DifficultySelectState::SelectingSize)),
                    systems::handle_select_wrap
                        .run_if(in_state(states::DifficultySelectState::SelectingWrap)),
                )
                    .distributive_run_if(in_state(GameState::SelectDifficulty)),
            )
            .add_systems(
                OnExit(states::DifficultySelectState::SelectingSize),
                despawn_entities::<components::OnSelectDifficulty>,
            )
            .add_systems(
                OnExit(states::DifficultySelectState::SelectingWrap),
                despawn_entities::<components::OnSelectDifficulty>,
            )
            .add_systems(
                OnExit(GameState::SelectDifficulty),
                despawn_entities::<components::OnSelectDifficulty>,
            );
    }
}
