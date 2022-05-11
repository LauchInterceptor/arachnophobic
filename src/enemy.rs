use crate::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Enemy;

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
            .insert(Enemy)
            .insert(Health { value: 50 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Sphere { radius: 8.0 })
            .insert(
                CollisionLayers::none()
                    .with_group(CollisionLayer::Enemy)
                    .with_masks(&[CollisionLayer::Player]),
            );
    }
}

pub fn enemy_die(mut commands: Commands, enemies: Query<(Entity, &Health), With<Enemy>>) {
    enemies.for_each(|enemy| {
        let (entity, health) = enemy;
        if health.value <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    });
}

pub fn enemy_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemies: Query<&mut Health, With<Enemy>>,
    mut damage_dealers: Query<(&Faction, &DealsDamage)>,
) {
    for event in collision_events.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();

        // match enemies.get_many_mut([e1, e2]) {
        //     Ok([h1, h2]) => {
        //         println!("{:?} {:?}", h1.value, h2.value);
        //     }
        //     Err(_) => (),
        // }

        if let Ok(mut health) = enemies.get_mut(e1) {
            if let Ok((faction, damage)) = damage_dealers.get(e2) {
                match faction {
                    Faction::Player => {
                        health.value -= damage.amount;
                    }
                    Faction::Spiders => (),
                }
            }
        } else if let Ok(mut health) = enemies.get_mut(e2) {
            if let Ok((faction, damage)) = damage_dealers.get(e1) {
                match faction {
                    Faction::Player => {
                        health.value -= damage.amount;
                    }
                    Faction::Spiders => (),
                }
            }
        }
    }
}
