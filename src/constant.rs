mod prelude {
    pub use crate::components::*;
}

use bevy::prelude::*;
use prelude::*;

impl Animation {
    pub const IDLE: Animation = Animation {
        path: "sprites/hero-idle-front.png",
        tile_size: Vec2::new(32., 32.),
        columns: 1,
        rows: 1,
    };
}