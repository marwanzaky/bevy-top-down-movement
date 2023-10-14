use bevy::prelude::*;

mod animation;
mod camera;

pub mod movement;

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::init_camera)
            .add_systems(Update, movement::transform)
            .add_systems(Update, movement::animation);
    }
}
