use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct MainTimer(pub Timer);
