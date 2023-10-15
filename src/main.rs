mod components;
mod constant;

mod prelude {
    pub use crate::components::*;
    pub use crate::constant::*;
}

use bevy::prelude::*;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (spawn_camera, spawn_player))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.25,

            ..Default::default()
        },
        ..default()
    });
}

fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation: Animation = Animation::IDLE;

    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(
        asset_server.load(animation.path),
        animation.tile_size,
        animation.columns,
        animation.rows,
        None,
        None,
    );

    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        Player {},
    ));
}
