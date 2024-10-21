use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct VirtualPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct SnakePart;

#[derive(Component)]
pub struct SnakeHead {
    pub locked: bool,
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeTail;
