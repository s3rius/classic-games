use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub struct FigurePlaced;

#[derive(Debug, Clone, Event)]
pub struct GameOver;

#[derive(Debug, Clone, Event)]
pub struct MoveTetronomioHorizontally {
    pub right: bool,
}

#[derive(Debug, Clone, Event)]
pub struct RotateTetronomio {
    pub clockwise: bool,
}

#[derive(Debug, Clone, Event)]
pub struct HardDrop;
