use crate::prelude::*;
use bevy::prelude::*;

pub fn get_animation(
    texture_atlases: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
    movement: &Vec3,
    indices: &mut AnimationIndices,
) -> Handle<TextureAtlas> {
    let mut columns: usize = 1;
    let mut path: &str= "hero-idle-front.png"; 
    
    *indices = AnimationIndices { first: 0, last: 0 };

    if movement.normalize().x > 0. {
        path = "hero-walk-side.png";
        columns = 6;
        *indices = AnimationIndices { first: 0, last: 5 };
    }

    if movement.normalize().x < 0. {
        path = "hero-walk-side-reverse.png";
        columns = 6;
        *indices = AnimationIndices { first: 0, last: 5 };
    }

    if movement.normalize().y > 0. {
        path = "hero-walk-back.png";
        columns = 6;
        *indices = AnimationIndices { first: 0, last: 5 };
    }

    if movement.normalize().y < 0. {
        path = "hero-walk-front.png";
        columns = 6;
        *indices = AnimationIndices { first: 0, last: 5 };
    }

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load(path),
        Vec2::new(32.0, 32.0),
        columns,
        1,
        None,
        None,
    );

    texture_atlases.add(texture_atlas)
}
