use crate::prelude::*;
use bevy::prelude::*;

pub fn transform(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let movement = movement(keyboard_input);
    let speed: f32 = 100.;

    if movement != Vec3::new(0., 0., 0.) {
        game.player.translation += movement.normalize() * speed * time.delta_seconds();
        game.player.animation_state = AnimationState::Walk;
    } else {
        game.player.animation_state = AnimationState::Idle;
    }

    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: game.player.translation,
        ..default()
    };
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
) -> Vec3 {
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