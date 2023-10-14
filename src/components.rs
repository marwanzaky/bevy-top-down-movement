use bevy::{prelude::*};

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub translation: Vec3,
    pub movement: Vec3,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);