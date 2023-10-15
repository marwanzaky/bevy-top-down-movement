use crate::prelude::*;
use bevy::prelude::*;

pub fn get_animation(
    direction: &Vec3,
) -> Animation {
    let mut animation: Animation = Animation::IDLE;

    if direction.x > 0. {
        animation = Animation::WALK_RIGHT;
    }

    if direction.x < 0. {
        animation = Animation::WALK_LEFT;
    }

    if direction.y > 0. {
        animation = Animation::WALK_UP;
    }

    if direction.y < 0. {
        animation = Animation::WALK_DOWN;
    }

    animation
}
