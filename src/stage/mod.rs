mod waves;

use crate::prelude::*;
use rand::{thread_rng, Rng};

use self::waves::*;
pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StageOrchestrationState>();
        app.add_enter_system(AppState::Game(Running), stage_startup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(Running))
                .with_system(on_enemy_death)
                .with_system(stage_orchestration)
                .into(),
        );
    }
}

pub struct StageOrchestrationState {
    pub wave: Option<Wave>,
    pub started: bool,
    pub second_timer: Timer,
    pub tick_timer: i32,
    pub current_action: Option<WaveAction>,
    pub alive_enemies: i32,
}

impl Default for StageOrchestrationState {
    fn default() -> Self {
        Self {
            wave: None,
            started: false,
            second_timer: Default::default(),
            tick_timer: 0,
            current_action: None,
            alive_enemies: 0,
        }
    }
}

pub fn stage_startup(mut state: ResMut<StageOrchestrationState>) {
    state.wave = Some(random_wave());
    state.started = true;
}

fn random_wave() -> Wave {
    let mut builder = WaveBuilder::new();
    for _ in 4..thread_rng().gen_range(10..16) {
        let rows: i32 = thread_rng().gen_range(1..4);
        let columns: i32 = thread_rng().gen_range(2..6);
        let enemy_type = match thread_rng().gen_range(0..3) {
            0 => EnemyType::MediumSpider,
            1 => EnemyType::SmallSpider,
            2 => EnemyType::TinySpider,
            _ => panic!("Bruh thats not an enemy"),
        };
        let next_spawn_delay: f32 = thread_rng().gen_range(1.0..3.0);

        let spacing = 96.0;
        let startx = -((((columns - 1) as f32) * spacing) / 2.0);
        let starty = (((rows - 1) as f32) * spacing) / 2.0;
        for column in 0..columns {
            for row in 0..rows {
                builder.spawn_at(
                    enemy_type,
                    Vec2::new(
                        startx + (column as f32 * spacing),
                        starty + (row as f32 * spacing),
                    ),
                );
            }
        }
        builder.wait_for(Condition::PreviousWaveHasDied);
        builder.wait_sec(next_spawn_delay);
    }
    builder.build()
}

pub fn on_enemy_death(
    mut enemy_death: EventReader<OnDeathEvent>,
    mut state: ResMut<StageOrchestrationState>,
) {
    for _ in enemy_death.iter() {
        state.alive_enemies -= 1;
    }
}

pub fn stage_orchestration(
    mut spawn_enemy: EventWriter<SpawnEnemyEvent>,
    mut state: ResMut<StageOrchestrationState>,
    time: Res<Time>,
) {
    if state.started {
        if let None = state.wave {
            error!("No wave supplied");
            return;
        }

        if !can_continue_orchestration(&mut state, &time) {
            return;
        }

        while let Some(action) = state.wave.as_mut().unwrap().next_action() {
            println!("{:?}", &action);
            state.current_action = Some(action);
            //borrow here
            match state.current_action.as_ref().unwrap() {
                WaveAction::Spawn(spawn) => match spawn {
                    EnemySpawn::At { enemy, position } => {
                        spawn_enemy.send(SpawnEnemyEvent {
                            position: position.clone(),
                            enemy_type: enemy.clone(),
                        });
                        state.alive_enemies += 1;
                    }
                    EnemySpawn::Inside => todo!(),
                },
                WaveAction::Delay(delay) => {
                    match delay {
                        Delay::Seconds(sec) => {
                            println!("Waiting for {} seconds", &sec);
                            state.second_timer = Timer::from_seconds(*sec, false);
                        }
                        Delay::Ticks(ticks) => state.tick_timer = *ticks,
                    };
                    break;
                }
                WaveAction::Condition(condition) => match condition {
                    Condition::PreviousWaveHasDied => break,
                    _ => todo!("not yet implemented!"),
                },
            }
        }
    }
}

fn can_continue_orchestration(state: &mut StageOrchestrationState, time: &Time) -> bool {
    if let Some(current_action) = &state.current_action {
        match current_action {
            WaveAction::Delay(Delay::Seconds(_)) => {
                // println!("{:.2} sec elapsed", state.second_timer.elapsed_secs());
                state.second_timer.tick(time.delta());
                if !state.second_timer.finished() {
                    // If timer hasn't run out, skip further orchestration
                    return false;
                }
            }
            WaveAction::Delay(Delay::Ticks(_)) => {
                state.tick_timer -= 1;
                if !state.tick_timer <= 0 {
                    // If timer hasn't run out, skip further orchestration
                    return false;
                }
            }
            WaveAction::Condition(condition) => {
                match condition {
                    Condition::PreviousWaveHasDied => {
                        if state.alive_enemies > 0 {
                            // If any enemies are alive don't continue spawning
                            return false;
                        }
                    }
                    Condition::And(_) => todo!("chaining conditions not yet implemented!"),
                    Condition::Or(_) => todo!("chaining conditions not yet implemented!"),
                };
            }
            _ => (),
        }
    }
    return true;
}
