use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OnMenuScreen;

#[derive(Component, Debug)]
pub enum ButtonAction {
    Quit,
    StartGame,
}
