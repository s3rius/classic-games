use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OnMenuScreen;

#[allow(dead_code)]
#[derive(Component, Debug)]
pub enum ButtonAction {
    Quit,
    StartGame,
}
