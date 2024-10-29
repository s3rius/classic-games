use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct SelectSize(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct SelectWrap(pub bool);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct OnSelectDifficulty;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct OnSelectSize;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct OnSelectWrap;
