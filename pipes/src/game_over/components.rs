use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OnGameOverScreen;

#[allow(dead_code)]
#[derive(Component, Debug)]
pub enum ButtonAction {
    Restart,
    ChangeDifficulty,
    Quit,
}
