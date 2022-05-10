use crate::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(InGame))
                .with_system(update_projectile)
                .with_system(spawn_projectile)
                .into(),
        );
    }
}

#[derive(Component)]
pub struct Projectile {
    // Velocity in pixels per second
    pub velocity: Vec2,
    pub faction: Faction,
}

pub enum Faction {
    Player,
    Spiders,
}

pub struct SpawnProjectileEvent {
    pub position: Vec3,
    pub rotation: Quat,
    pub speed: f32,
}

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
                faction: Faction::Player,
            },
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
                faction: Faction::Player,
            },
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
