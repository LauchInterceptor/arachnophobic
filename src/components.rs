use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Component)]
pub struct DealsDamage {
    pub amount: i32,
}

#[derive(Component)]
pub enum Faction {
    Player,
    Spiders,
}
