mod components;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;
}

use bevy::prelude::*;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_plugins(GamePlugins)
        .add_systems(Update, (coin_animation, collect_coin))
        .run();
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    spawn_coin(
        &asset_server,
        &mut commands,
        &mut texture_atlases,
        Vec3 {
            x: 100.,
            y: 0.,
            z: 0.,
        },
    );
    spawn_coin(
        &asset_server,
        &mut commands,
        &mut texture_atlases,
        Vec3 {
            x: -100.,
            y: 0.,
            z: 0.,
        },
    );
    spawn_coin(
        &asset_server,
        &mut commands,
        &mut texture_atlases,
        Vec3 {
            x: 0.,
            y: 50.,
            z: 0.,
        },
    );
    spawn_coin(
        &asset_server,
        &mut commands,
        &mut texture_atlases,
        Vec3 {
            x: 0.,
            y: -50.,
            z: 0.,
        },
    );

    spawn_coin_text(&mut commands);
}

fn spawn_coin_text(commands: &mut Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Coins: ",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
        ]),
        CoinText {
            text: "0".to_string(),
            total: 0,
        },
    ));
}

fn spawn_coin(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3,
) {
    let texture_handle = asset_server.load("sprites/coin.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(5.0, 7.0), 7, 1, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        Coin {
            name: "Coin".to_string(),
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_translation(translation),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn coin_animation(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Coin>,
    >,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn collect_coin(
    mut commands: Commands,
    coin_query: Query<
        (Entity, &Transform),
        (
            With<Coin>,
            With<AnimationIndices>,
            With<AnimationTimer>,
            With<TextureAtlasSprite>,
        ),
    >,
    player_query: Query<
        &Transform,
        (
            Without<Coin>,
            With<AnimationIndices>,
            With<AnimationTimer>,
            With<TextureAtlasSprite>,
        ),
    >,
    mut coin_text_query: Query<(&mut Text, &mut CoinText), With<CoinText>>,
) {
    let collect_coin_dis = 10.;

    for player_transform in &player_query {
        for (coin_entity, coin_transform) in &coin_query {
            let dis = player_transform
                .translation
                .distance(coin_transform.translation);

            if dis <= collect_coin_dis {
                commands.entity(coin_entity).despawn();

                for  (mut text, mut coin_text) in &mut coin_text_query {
                    coin_text.total = coin_text.total + 1;
                    text.sections[1].value = coin_text.total.to_string();
                }
            }
        }
    }
}
