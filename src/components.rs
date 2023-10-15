use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub translation: Vec3,
    pub direction: Vec3,
    pub animation_kind: AnimationKind,
}

#[derive(Component)]
pub struct Coin {
    pub name: String,
}

#[derive(Component)]
pub struct CoinText {
    pub text: String,
    pub total: i32,
}

#[derive(Component, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default, PartialEq)]
pub enum AnimationKind {
    #[default]
    Idle,
    WalkRight,
    WalkLeft,
    WalkUp,
    WalkDown,
}

#[derive(Component, Default)]
pub struct Animation {
    pub kind: AnimationKind,
    pub path: &'static str,
    pub columns: usize,
    pub indices: AnimationIndices,
}

impl Animation {
    pub const IDLE: Animation = Animation {
        kind: AnimationKind::Idle,
        path: "sprites/hero-idle-front.png",
        columns: 1,
        indices: AnimationIndices { first: 0, last: 0 },
    };

    pub const WALK_RIGHT: Animation = Animation {
        kind: AnimationKind::WalkRight,
        path: "sprites/hero-walk-side.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };

    pub const WALK_LEFT: Animation = Animation {
        kind: AnimationKind::WalkLeft,
        path: "sprites/hero-walk-side-reverse.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };

    pub const WALK_UP: Animation = Animation {
        kind: AnimationKind::WalkUp,
        path: "sprites/hero-walk-back.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };

    pub const WALK_DOWN: Animation = Animation {
        kind: AnimationKind::WalkDown,
        path: "sprites/hero-walk-front.png",
        columns: 6,
        indices: AnimationIndices { first: 0, last: 5 },
    };
}
