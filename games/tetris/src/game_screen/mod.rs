use std::time::Duration;

use bevy::prelude::*;

use crate::{state::GameState, utils::systems::despawn_screen};

pub struct GameScreenPlugin;

mod components;
mod debug;
mod events;
mod resources;
mod systems;
mod timers;

impl Plugin for GameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                systems::setup_layout,
                systems::reset_game,
                systems::spawn_figure,
            ),
        )
        .init_resource::<resources::TetroBag>()
        .insert_resource(resources::GameBoard::new())
        .insert_resource(timers::GravityTimer(Timer::new(
            Duration::from_secs_f64(0.4),
            TimerMode::Repeating,
        )))
        .insert_resource(timers::LockdownTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        .insert_resource(resources::SoftDrop::default())
        .add_event::<events::FigurePlaced>()
        .add_event::<events::GameOver>()
        .add_event::<events::MoveTetronomioHorizontally>()
        .add_event::<events::RotateTetronomio>()
        .add_event::<events::HardDrop>()
        .add_systems(
            Update,
            (
                // Logical systems
                systems::color_grid,
                systems::update_score_label,
                systems::handle_inputs,
                systems::gravity,
                systems::lockdown_tiles,
                systems::clear_lines,
                // Control systemset. It runs after handle_inputs
                // to reduce latency between input and action.
                (
                    systems::move_tetronomio_horizontally
                        .run_if(on_event::<events::MoveTetronomioHorizontally>()),
                    systems::rotate_tetronomio.run_if(on_event::<events::RotateTetronomio>()),
                    systems::hard_drop
                        .run_if(on_event::<events::HardDrop>())
                        .before(systems::gravity),
                )
                    .after(systems::handle_inputs),
                // Debug systemset.
                (
                    debug::center_point.after(systems::color_grid),
                    debug::stop_gravity,
                ),
            )
                .distributive_run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (
                despawn_screen::<components::OnGameScreen>,
                despawn_screen::<components::PlayableTile>,
            ),
        );
    }
}
