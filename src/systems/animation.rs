use crate::prelude::*;
use bevy::prelude::*;

pub fn get_animation(
    texture_atlases: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
    movement: &Vec3,
    indices: &mut AnimationIndices,
) -> Handle<TextureAtlas> {
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

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load(animation.path),
        Vec2::new(32.0, 32.0),
        animation.columns,
        1,
        None,
        None,
    );

    *indices = animation.indices;

    texture_atlases.add(texture_atlas)
}
