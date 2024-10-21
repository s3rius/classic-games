use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OnDeathScreen;

#[derive(Component, Debug)]
pub enum ButtonAction {
    Quit,
    StartGame,
}
