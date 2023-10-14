use bevy::prelude::*;

#[derive(Resource, Default)]
struct Game {
    player: Player,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    translation: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    game.player.translation = Vec3::new(0., 0., 0.);

    commands.spawn(Camera2dBundle::default());

    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("icon.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .id(),
    );
}

fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut movement: Vec3 = Vec3::new(0., 0., 0.);
    let speed: f32 = 150.;

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

    if movement != Vec3::new(0., 0., 0.) {
        game.player.translation += movement.normalize() * speed * time.delta_seconds();
    }

    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: game.player.translation,
        ..default()
    };
}
