use std::ops::Sub;

use crate::prelude::*;

use crate::{TextureAtlasAssets, WINDOW_SIZE};

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
}

pub struct Hardpoint {}

#[derive(Component)]
pub struct Armed {
    pub hardpoints: Vec<Hardpoint>,
    pub rolling_fire: bool,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub roll_frame: i8,
}

pub fn spawn_player(mut commands: Commands, texture_atlas_assets: Res<TextureAtlasAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_assets.player_ship.clone(),
            sprite: TextureAtlasSprite::new(5),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..Default::default()
        })
        .insert(Player {
            movement_speed: 500.0,
        })
        .insert(PlayerAnimation { roll_frame: 5 })
        .insert(RigidBody::KinematicPositionBased)
        .insert(CollisionShape::Sphere { radius: 64.0 })
        .insert(
            CollisionLayers::none()
                .with_group(CollisionLayer::Player)
                .with_masks(&[CollisionLayer::Enemy]),
        );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut PlayerAnimation)>,
) {
    if let Ok((ship, mut transform, mut player_animation)) = query.get_single_mut() {
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

        if movement.x < 0.0 {
            player_animation.roll_frame = (player_animation.roll_frame + 1).min(10);
        } else if movement.x > 0.0 {
            player_animation.roll_frame = (player_animation.roll_frame - 1).max(0);
        } else {
            player_animation.roll_frame -= player_animation.roll_frame.sub(5).signum();
        }

        // limit movement
        limit_player_movement(&mut transform);
    }
}

pub fn player_shoot(
    keyboard_input: Res<Input<KeyCode>>,
    mut spawn_projectile: EventWriter<SpawnProjectileEvent>,
    query: Query<(&Player, &Transform, &PlayerAnimation)>,
) {
    if let Ok((_player, transform, _player_animation)) = query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) {
            spawn_projectile.send(SpawnProjectileEvent {
                position: transform.translation.clone(),
                rotation: transform.rotation.clone(),
                speed: 1000.0 * crate::TIME_STEP,
            });
        }
    }
}

pub fn limit_player_movement(transform: &mut Transform) {
    transform.translation = transform
        .translation
        .min(Vec3::from((WINDOW_SIZE.x / 2.0, 0.0, 0.0)))
        .max(Vec3::from((
            -WINDOW_SIZE.x / 2.0,
            -WINDOW_SIZE.y / 2.0,
            0.0,
        )));
}

pub fn animate_player(mut query: Query<(&PlayerAnimation, &mut TextureAtlasSprite)>) {
    query.for_each_mut(|(player_animation, mut sprite)| {
        sprite.index = player_animation.roll_frame as usize;
    });
}
