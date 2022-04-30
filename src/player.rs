use bevy::prelude::*;

use crate::WINDOW_SIZE;

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
}

pub fn spawn_player(commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            movement_speed: 500.0,
        });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    let mut movement = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        movement.y += ship.movement_speed;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        movement.y -= ship.movement_speed;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        movement.x -= ship.movement_speed;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        movement.x += ship.movement_speed;
    }

    transform.translation += Vec3::from((movement, 0.0)) * crate::TIME_STEP;

    // limit movement
    limit_player_movement(&mut transform);

    println!("{:?}", transform.translation);
}

fn limit_player_movement(transform: &mut Transform) {
    transform.translation = transform
        .translation
        .min(Vec3::from((WINDOW_SIZE.x / 2.0, 0.0, 0.0)))
        .max(Vec3::from((
            -WINDOW_SIZE.x / 2.0,
            -WINDOW_SIZE.y / 2.0,
            0.0,
        )));
}
