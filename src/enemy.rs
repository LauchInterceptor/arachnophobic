use crate::prelude::*;
use rand::{thread_rng, Rng};

pub struct SpawnEnemyEvent {
    pub position: Vec2,
}

pub struct StageOrchestrationState {
    grid: [bool; 16],
    next_spawn: Timer,
}

impl Default for StageOrchestrationState {
    fn default() -> Self {
        Self {
            grid: [false; 16],
            next_spawn: Timer::from_seconds(1.0, true),
        }
    }
}

pub fn enemy_orchestration(
    mut event: EventWriter<SpawnEnemyEvent>,
    mut stage_orchestration_state: ResMut<StageOrchestrationState>,
    time: Res<Time>,
) {
    if !stage_orchestration_state.grid.iter().any(|&x| x == false) {
        return;
    }
    stage_orchestration_state.next_spawn.tick(time.delta());
    if stage_orchestration_state.next_spawn.finished() {
        loop {
            let index = thread_rng().gen_range(0..16);
            if !stage_orchestration_state.grid[index] {
                let x = (((index % 4) as i32 * 125) - 250) as f32;
                let y = ((index / 4) as i32 * 96) as f32;
                event.send(SpawnEnemyEvent {
                    position: Vec2::new(x, y),
                });
                stage_orchestration_state.grid[index] = true;
                stage_orchestration_state.next_spawn.reset();
                break;
            }
        }
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut event: EventReader<SpawnEnemyEvent>,
    sprite_assets: Res<SpriteAssets>,
) {
    for spawn_enemy_event in event.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..Default::default()
                },
                texture: sprite_assets.spider_tiny.clone(),
                transform: Transform::from_translation(Vec3::from((
                    spawn_enemy_event.position,
                    0.0,
                ))),
                ..Default::default()
            })
            .insert(Health { health: 50 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Sphere { radius: 8.0 })
            .insert(
                CollisionLayers::none()
                    .with_group(CollisionLayer::Enemy)
                    .with_masks(&[CollisionLayer::Player]),
            );
    }
}
