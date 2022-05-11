use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Component)]
pub struct DealsContactDamage {
    pub amount: i32,
}

#[derive(Component)]
pub struct NotifyDeath;

pub struct OnDeathEvent {
    pub entity: Entity,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnDeathEvent>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(InGame))
                .with_system(death_system)
                .with_system(contact_damage)
                .into(),
        );
    }
}

/**
 * Despawns any entities which Health components have reached zero
 * Optionally emits an NotifyDeath event for that entity
 */
pub fn death_system(
    mut commands: Commands,
    enemies: Query<(Entity, &Health, Option<&NotifyDeath>)>,
    mut notify_death: EventWriter<OnDeathEvent>,
) {
    enemies.for_each(|enemy| {
        let (entity, health, notify) = enemy;
        if health.value <= 0 {
            commands.entity(entity).despawn_recursive();
            if let Some(_) = notify {
                notify_death.send(OnDeathEvent { entity });
            }
        }
    });
}

pub fn contact_damage(
    mut collision_events: EventReader<CollisionEvent>,
    mut healthy: Query<&mut Health>,
    damage_dealers: Query<(&Faction, &DealsContactDamage)>,
) {
    for event in collision_events.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();

        if let Ok(mut health) = healthy.get_mut(e1) {
            if let Ok((faction, damage)) = damage_dealers.get(e2) {
                if let Faction::Player = faction {
                    health.value -= damage.amount;
                }
            }
        } else if let Ok(mut health) = healthy.get_mut(e2) {
            if let Ok((faction, damage)) = damage_dealers.get(e1) {
                if let Faction::Player = faction {
                    health.value -= damage.amount;
                }
            }
        }
    }
}
