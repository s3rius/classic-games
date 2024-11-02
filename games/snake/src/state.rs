use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub usize);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
    GameOver,
}
