use bevy::prelude::*;

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct GravityTimer(pub Timer);

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct LockdownTimer(pub Timer);
