mod components;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;
}

use bevy::{prelude::*};
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_plugins(GamePlugins)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    game.player.translation = Vec3::new(0., 0., 0.);

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("hero-idle-front.png"),
        Vec2::new(32.0, 32.0),
        7,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 0 };

    game.player.entity = Some(
        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ))
            .id(),
    );
}
