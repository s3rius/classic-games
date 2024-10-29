use bevy::{prelude::*, time::Stopwatch};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartMenu,
    SelectDifficulty,
    Playing,
    GameOver,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct GameDifficulty {
    pub size: usize,
    pub wrap: bool,
}

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct SoltutionTimer(pub Stopwatch);
