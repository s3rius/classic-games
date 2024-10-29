use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Component)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Component)]
pub struct PipeSprite;
