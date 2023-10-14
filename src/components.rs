use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub translation: Vec3,
    pub movement: Vec3,
}

#[derive(Component, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default)]
pub struct Animation {
    pub path: &'static str,
    pub columns: usize,
    pub indices: AnimationIndices,
}

impl Animation {
    pub const IDLE: Animation = Animation {
        path: "hero-idle-front.png",
        columns: 1,
        indices: AnimationIndices { first: 0, last: 0 },
    };

    pub const WALK_RIGHT: Animation = Animation {
        path: "hero-walk-side.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };

    pub const WALK_LEFT: Animation = Animation {
        path: "hero-walk-side-reverse.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };

    pub const WALK_UP: Animation = Animation {
        path: "hero-walk-back.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };

    pub const WALK_DOWN: Animation = Animation {
        path: "hero-walk-front.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };
}
