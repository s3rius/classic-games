use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OnGameOverScreen;

#[derive(Component, Debug)]
pub enum ButtonAction {
    Restart,
    ChangeDifficulty,
    Quit,
}
