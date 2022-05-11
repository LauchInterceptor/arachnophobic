use bevy_asset_loader::AssetCollection;

use crate::prelude::*;

#[derive(Bundle)]
pub struct GenericEnemyBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub enemy: Enemy,
    pub notify_death: NotifyDeath,
    pub health: Health,
    pub rigidbody: RigidBody,
    pub collision_shape: CollisionShape,
    pub collision_layers: CollisionLayers,
}

impl Default for GenericEnemyBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            enemy: Enemy,
            notify_death: NotifyDeath,
            health: Health { value: 1 },
            rigidbody: RigidBody::KinematicPositionBased,
            collision_shape: CollisionShape::Sphere { radius: 8.0 },
            collision_layers: CollisionLayers::none()
                .with_group(CollisionLayer::Enemy)
                .with_masks(&[CollisionLayer::Player]),
        }
    }
}

#[derive(AssetCollection)]
pub struct EnemyFactory {
    #[asset(path = "enemy/spider_medium.png")]
    pub spider_medium: Handle<Image>,
    #[asset(path = "enemy/spider_small.png")]
    pub spider_small: Handle<Image>,
    #[asset(path = "enemy/spider_tiny.png")]
    pub spider_tiny: Handle<Image>,
}

impl EnemyFactory {
    pub fn tiny_spider(&self) -> GenericEnemyBundle {
        GenericEnemyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..Default::default()
                },
                texture: self.spider_tiny.clone(),
                transform: Transform::from_translation(Vec3::from((Vec2::ZERO, 0.0))),
                ..Default::default()
            },
            health: Health { value: 15 },
            ..Default::default()
        }
    }

    pub fn small_spider(&self) -> GenericEnemyBundle {
        GenericEnemyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                texture: self.spider_tiny.clone(),
                transform: Transform::from_translation(Vec3::from((Vec2::ZERO, 0.0))),
                ..Default::default()
            },
            health: Health { value: 50 },
            ..Default::default()
        }
    }

    pub fn medium_spider(&self) -> GenericEnemyBundle {
        GenericEnemyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                texture: self.spider_tiny.clone(),
                transform: Transform::from_translation(Vec3::from((Vec2::ZERO, 0.0))),
                ..Default::default()
            },
            health: Health { value: 200 },
            ..Default::default()
        }
    }
}
