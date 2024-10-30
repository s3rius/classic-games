use bevy::{prelude::*, time::Stopwatch};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
    GameOver,
}
