use bevy::prelude::*;

use crate::globals::GameState;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, SubStates)]
#[source(GameState = GameState::SelectDifficulty)]
pub enum DifficultySelectState {
    #[default]
    SelectingSize,
    SelectingWrap,
}
