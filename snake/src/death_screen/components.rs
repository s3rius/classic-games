use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OnDeathScreen;

#[allow(dead_code)]
#[derive(Component, Debug)]
pub enum ButtonAction {
    Quit,
    StartGame,
}
