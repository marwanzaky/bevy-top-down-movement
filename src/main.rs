use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("icon.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut direction, mut transform) in &mut sprite_position {
        let mut movement: Vec3 = Vec3::new(0., 0., 0.);
        let speed: f32 = 150.;

        if keyboard_input.pressed(KeyCode::Right) {
            movement.x += 1.;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            movement.x -= 1.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            movement.y += 1.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            movement.y -= 1.;
        }

        if movement != Vec3::new(0., 0., 0.) {
            transform.translation += movement.normalize() * speed * time.delta_seconds();
        }
    }
}
