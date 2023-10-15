use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    
}

#[derive(Component)]
pub struct Animation {
    pub path: &'static str,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
}