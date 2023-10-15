use crate::prelude::*;
use bevy::prelude::*;

pub fn get_animation(
    movement: &Vec3,
) -> Animation {
    let mut animation: Animation = Animation::IDLE;

    if movement.normalize().x > 0. {
        animation = Animation::WALK_RIGHT;
    }

    if movement.normalize().x < 0. {
        animation = Animation::WALK_LEFT;
    }

    if movement.normalize().y > 0. {
        animation = Animation::WALK_UP;
    }

    if movement.normalize().y < 0. {
        animation = Animation::WALK_DOWN;
    }

    animation
}
