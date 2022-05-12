use bevy::math::{const_vec2, Vec3Swizzles};

use crate::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(Running))
                .with_system(update_projectile)
                .with_system(spawn_projectile)
                .with_system(projectile_collision)
                .with_system(despawn_projectile)
                .into(),
        );
    }
}

#[derive(Component)]
pub struct Projectile {
    // Velocity in pixels per second
    pub velocity: Vec2,
}

pub struct SpawnProjectileEvent {
    pub position: Vec3,
    pub rotation: Quat,
    pub speed: f32,
}
// TODO: move Element and ProjectileType somewhere more sensible
pub enum Element {
    Water,
    Fire,
    Earth,
    Air,
}

pub enum ProjectileType {
    Single(Element),
    Compound(Element, Element),
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub projectile: Projectile,
    pub faction: Faction,
    pub damage: DealsContactDamage,
    pub rigidbody: RigidBody,
    pub collider: CollisionShape,
    pub collision_layer: CollisionLayers,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                texture: Default::default(),
                visibility: Default::default(),
            },
            rigidbody: RigidBody::KinematicPositionBased,
            projectile: Projectile {
                velocity: Vec2::new(0.0, 1.0),
            },
            damage: DealsContactDamage { amount: 0 },
            faction: Faction::Player,
            collider: CollisionShape::Sphere { radius: 1.0 },
            collision_layer: CollisionLayers::all::<CollisionLayer>(),
        }
    }
}

pub fn spawn_projectile(
    mut commands: Commands,
    mut event: EventReader<SpawnProjectileEvent>,
    sprite_assets: Res<SpriteAssets>,
) {
    for spawn_projectile_event in event.iter() {
        commands.spawn_bundle(ProjectileBundle {
            projectile: Projectile {
                velocity: Vec2::new(0.0, spawn_projectile_event.speed),
            },
            damage: DealsContactDamage { amount: 25 },
            faction: Faction::Player,
            collider: CollisionShape::Sphere { radius: 4.0 },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.32, 0.32, 1.),
                    ..Default::default()
                },
                transform: Transform {
                    translation: spawn_projectile_event.position,
                    rotation: spawn_projectile_event.rotation,
                    ..Default::default()
                },
                texture: sprite_assets.projectile.clone(),
                ..Default::default()
            },
            collision_layer: CollisionLayers::none()
                .with_group(CollisionLayer::Player)
                .with_masks(&[CollisionLayer::Enemy]),
            ..Default::default()
        });
    }
}

pub fn update_projectile(mut query: Query<(&Projectile, &mut Transform)>) {
    query.for_each_mut(|(projectile, mut transform)| {
        transform.translation += Vec3::from((projectile.velocity, 0.0));
    });
}

pub fn despawn_projectile(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    const BOUNDS: Vec2 = const_vec2!([500.0, 500.0]);

    query.for_each(|(entity, transform)| {
        // If tranlation is outside of bounds on any axis
        let xy = transform.translation.xy();

        if xy.cmpgt(BOUNDS).any() || xy.cmplt(-BOUNDS).any() {
            commands.entity(entity).despawn_recursive();
        }
    });
}

pub fn projectile_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectiles: Query<&Projectile>,
) {
    for event in collision_events.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();
        // Check if one of the entities has a projectile component
        // If it collided destroy the projectile
        if projectiles.get_component::<Projectile>(e1).is_ok() {
            commands.entity(e1).despawn_recursive();
        } else if projectiles.get_component::<Projectile>(e2).is_ok() {
            commands.entity(e2).despawn_recursive();
        }
    }
}
