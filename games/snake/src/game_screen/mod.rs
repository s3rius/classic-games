use std::time::Duration;

use crate::{consts, state::GameState};
use bevy::prelude::*;
use events::{EatingEvent, GameOverEvent};
use resources::MainTimer;

mod components;
mod events;
mod resources;
mod systems;

pub struct GameScreenPlugin;

impl Plugin for GameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatingEvent>()
            .add_event::<GameOverEvent>()
            .insert_resource(MainTimer(Timer::new(
                Duration::from_secs_f64(consts::TICK_DURATION),
                TimerMode::Repeating,
            )))
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    systems::spawn_food,
                    systems::spawn_snake,
                    systems::reset_score,
                    systems::reset_timer,
                ),
            )
            .add_systems(
                Update,
                (
                    systems::handle_input,
                    systems::check_on_food,
                    systems::check_on_cannibalism,
                    systems::check_on_edge_running,
                    systems::move_snake,
                    systems::position_translation,
                    systems::eat_food.run_if(on_event::<events::EatingEvent>()),
                    systems::grow_snake.run_if(on_event::<events::EatingEvent>()),
                    systems::game_over.run_if(on_event::<events::GameOverEvent>()),
                )
                    .distributive_run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), systems::despawn_snake);
    }
}
