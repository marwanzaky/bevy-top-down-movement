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
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    game.player.translation = Vec3::new(0., 0., 0.);

    game.player.entity = Some(
        commands
            .spawn((
                SpriteSheetBundle::default(),
                AnimationIndices::default(),
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ))
            .id(),
    );
}
