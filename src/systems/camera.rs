use bevy::prelude::*;

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.2,

            ..Default::default()
        },
        ..default()
    });
}
