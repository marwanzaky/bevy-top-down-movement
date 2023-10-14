use crate::prelude::*;
use bevy::prelude::*;

use super::animation::get_animation;

pub fn transform(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let movement: Vec3 = movement(keyboard_input);
    let speed: f32 = 100.;

    if movement != Vec3::new(0., 0., 0.) {
        game.player.translation += movement.normalize() * speed * time.delta_seconds();
        game.player.movement = movement;
    } else {
        game.player.movement = Vec3::new(0., 0., 0.);
    }

    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: game.player.translation,
        ..default()
    };
}

pub fn animation(
    time: Res<Time>,
    game: ResMut<Game>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for (mut indices, mut timer, mut sprite, mut texture) in &mut query {
        timer.tick(time.delta());

        *texture = get_animation(
            &mut texture_atlases,
            &asset_server,
            &game.player.movement,
            &mut indices,
        );

        if sprite.index > indices.last {
            sprite.index = 0;
        }

        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn movement(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut movement: Vec3 = Vec3::new(0., 0., 0.);

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        movement.x += 1.;
    }

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        movement.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        movement.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        movement.y -= 1.;
    }

    movement
}
