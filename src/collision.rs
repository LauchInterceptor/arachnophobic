use crate::prelude::*;
#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    Enemy,
}

pub fn collision_system(mut commands: Commands, mut collision_event: EventReader<CollisionEvent>) {
    for event in collision_event.iter() {}
}
