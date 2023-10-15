use crate::prelude::*;
use bevy::prelude::*;

use super::animation::get_animation;

pub fn transform(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let direction: Vec3 = get_direction(keyboard_input);
    let speed: f32 = 100.;

    game.player.translation += direction * speed * time.delta_seconds();
    game.player.direction = direction;
    
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: game.player.translation,
        ..default()
    };
}

pub fn animation(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        Without<Coin>,
    >,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for (mut indices, mut timer, mut sprite, mut texture) in &mut query {
        let animation: Animation = get_animation(&game.player.direction);

        *indices = animation.indices;

        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load(animation.path),
            Vec2::new(32.0, 32.0),
            animation.columns,
            1,
            None,
            None,
        );

        *texture = texture_atlases.add(texture_atlas);

        if game.player.animation_kind != animation.kind {
            sprite.index = indices.first;
        }

        game.player.animation_kind = animation.kind;

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

fn get_direction(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction: Vec3 = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.;
    }

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.;
    }

    if direction.length() > 0. {
        return direction.normalize()
    }

    direction
}
