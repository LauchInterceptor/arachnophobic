use crate::prelude::*;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Projectile {
                velocity: Vec2::new(0.0, 1.0),
            },
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: Default::default(),
            visibility: Default::default(),
        }
    }
}

pub struct SpawnProjectileEvent {
    pub position: Vec3,
    pub rotation: Quat,
    pub speed: f32,
}

#[derive(Component)]
pub struct Projectile {
    // Velocity in pixels per second
    pub velocity: Vec2,
}

pub fn spawn_projectile(
    mut commands: Commands,
    mut event: EventReader<SpawnProjectileEvent>,
    sprite_assets: Res<SpriteAssets>,
) {
    for spawn_projectile_event in event.iter() {
        commands.spawn_bundle(ProjectileBundle {
            sprite: Sprite {
                color: Color::rgb(0.32, 0.32, 1.),
                ..Default::default()
            },
            projectile: Projectile {
                velocity: Vec2::new(0.0, spawn_projectile_event.speed),
            },
            transform: Transform {
                translation: spawn_projectile_event.position,
                rotation: spawn_projectile_event.rotation,
                ..Default::default()
            },
            texture: sprite_assets.projectile.clone(),
            ..Default::default()
        });
    }
}

pub fn update_projectile(mut query: Query<(&Projectile, &mut Transform)>) {
    query.for_each_mut(|(projectile, mut transform)| {
        transform.translation += Vec3::from((projectile.velocity, 0.0));
    });
}
