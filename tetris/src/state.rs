use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Resource)]
pub struct Score {
    pub score: usize,
    pub lines_cleared: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Resource)]
pub struct Debug {
    pub enabled: bool,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
    GameOver,
}
