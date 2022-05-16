use std::collections::{
    vec_deque::{IntoIter, Iter},
    VecDeque,
};

use bevy::math::Vec2;

use crate::prelude::EnemyType;

#[derive(Debug)]
pub enum EnemySpawn {
    At { enemy: EnemyType, position: Vec2 },
    Inside,
}

#[derive(Debug)]
pub enum Delay {
    Seconds(f32),
    Ticks(i32),
}

#[derive(Debug)]
pub enum Condition {
    PreviousWaveHasDied,
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

#[derive(Debug)]
pub enum WaveAction {
    Spawn(EnemySpawn),
    Delay(Delay),
    Condition(Condition),
}

/**
 * A wave of enemies
 */
pub struct Wave {
    actions: VecDeque<WaveAction>,
}

impl Wave {
    pub fn next_action(&mut self) -> Option<WaveAction> {
        self.actions.pop_front()
    }
}

impl IntoIterator for Wave {
    type Item = WaveAction;

    type IntoIter = IntoIter<WaveAction>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}

impl From<Vec<WaveAction>> for Wave {
    fn from(actions: Vec<WaveAction>) -> Self {
        Self {
            actions: VecDeque::from(actions),
        }
    }
}

/**
 * You can build waves of enemies using a fluent syntax
 */
pub struct WaveBuilder {
    actions: Vec<WaveAction>,
}

impl WaveBuilder {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

impl WaveBuilder {
    /**
     * Spawn a single enemy
     */
    #[inline]
    pub fn spawn_at(&mut self, enemy: EnemyType, position: Vec2) -> &WaveBuilder {
        self.actions
            .push(WaveAction::Spawn(EnemySpawn::At { enemy, position }));
        self
    }

    /**
     * Wait for condition to be true
     */
    #[inline]
    pub fn wait_for(&mut self, condition: Condition) -> &WaveBuilder {
        self.actions.push(WaveAction::Condition(condition));
        self
    }

    /**
     * Wait seconds until next spawn
     */
    #[inline]
    pub fn wait_sec(&mut self, seconds: f32) -> &WaveBuilder {
        self.actions
            .push(WaveAction::Delay(Delay::Seconds(seconds)));
        self
    }

    /**
     * Wait ticks until next spawn
     */
    #[inline]
    pub fn wait_ticks(&mut self, ticks: i32) -> &WaveBuilder {
        self.actions.push(WaveAction::Delay(Delay::Ticks(ticks)));
        self
    }

    pub fn build(self) -> Wave {
        Wave {
            actions: VecDeque::from(self.actions),
        }
    }
}
