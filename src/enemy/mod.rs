mod factory;
pub mod types;

use bevy_asset_loader::AssetCollectionApp;

pub use factory::*;
pub use types::*;

use crate::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub struct SpawnEnemyEvent {
    pub position: Vec2,
    pub enemy_type: EnemyType,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<EnemyFactory>();
        app.add_event::<SpawnEnemyEvent>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(Running))
                .with_system(spawn_enemy)
                .with_system(on_death)
                .into(),
        );
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
            _ => todo!(),
        };
        enemy_bundle.sprite.transform =
            Transform::from_translation(Vec3::from((spawn_enemy_event.position, 0.0)));
        commands.spawn_bundle(enemy_bundle);
    }
}

pub fn on_death(mut death_event: EventReader<OnDeathEvent>, query: Query<Entity, With<Enemy>>) {
    for event in death_event.iter() {
        if let entity = query.get(event.entity) {
            println!("Notified enemy {:?} on death", entity);
        }
    }
}
