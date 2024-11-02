use bevy::prelude::Event;

use crate::utils::direction::Direction;

#[derive(Debug, Clone, Copy, Event)]
pub struct ChangeFocusEvent(pub Direction);

#[derive(Debug, Default, Clone, Copy, Event)]
pub struct RotatePipeEvent;

#[derive(Debug, Default, Clone, Copy, Event)]
pub struct PipeCompletedEvent;
