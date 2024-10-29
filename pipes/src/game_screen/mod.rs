use bevy::prelude::*;

use crate::{globals::GameState, utils::systems::despawn_entities};

mod componenets;
mod events;
pub mod game_board;
mod systems;

pub struct GameScreenPlugin;

impl Plugin for GameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::ChangeFocusEvent>()
            .add_event::<events::RotatePipeEvent>()
            .add_event::<events::PipeCompletedEvent>()
            .insert_resource(game_board::GameBoard::new())
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    systems::generate_game_board,
                    systems::render_grid,
                    systems::setup_camera,
                    systems::restart_timer,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    systems::camera_follow_cursor,
                    systems::controls,
                    (systems::highlight_connected, systems::highlight_focused).chain(),
                    systems::fix_rotations,
                    systems::check_on_completed,
                    systems::rotate_focused.run_if(on_event::<events::RotatePipeEvent>()),
                    systems::change_focus.run_if(on_event::<events::ChangeFocusEvent>()),
                )
                    .distributive_run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                despawn_entities::<componenets::PipeSprite>,
            );
    }
}
