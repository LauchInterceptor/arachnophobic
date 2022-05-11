mod factory;
mod types;

use bevy_asset_loader::AssetCollectionApp;

use factory::*;
use types::*;

use crate::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Enemy;

pub struct SpawnEnemyEvent {
    pub position: Vec2,
    pub enemy_type: EnemyType,
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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<EnemyFactory>();
        app.add_event::<SpawnEnemyEvent>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(InGame))
                .with_system(spawn_enemy)
                .with_system(enemy_orchestration)
                .with_system(on_death)
                .into(),
        );
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
                    enemy_type: EnemyType::SmallSpider,
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
    enemy_factory: Res<EnemyFactory>,
) {
    for spawn_enemy_event in event.iter() {
        let mut enemy_bundle = match spawn_enemy_event.enemy_type {
            EnemyType::TinySpider => enemy_factory.tiny_spider(),
            EnemyType::SmallSpider => enemy_factory.small_spider(),
            EnemyType::MediumSpider => enemy_factory.medium_spider(),
        };
        enemy_bundle.sprite.transform =
            Transform::from_translation(Vec3::from((spawn_enemy_event.position, 0.0)));
        commands.spawn_bundle(enemy_bundle);
    }
}

pub fn on_death(mut death_event: EventReader<OnDeathEvent>, query: Query<Entity, With<Enemy>>) {
    for event in death_event.iter() {
        if let entity = query.get(event.entity) {
            println!("Notified enemy on death");
        }
    }
}
