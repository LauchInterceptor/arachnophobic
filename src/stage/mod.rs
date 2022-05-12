use crate::prelude::*;
use rand::{thread_rng, Rng};
pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StageOrchestrationState>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game(Running))
                .with_system(stage_orchestration)
                .into(),
        );
    }
}

pub struct StageOrchestrationState {
    grid: [bool; 16],
    next_spawn: Timer,
}

impl Default for StageOrchestrationState {
    fn default() -> Self {
        Self {
            grid: [false; 16],
            next_spawn: Timer::from_seconds(1.0, true),
        }
    }
}

pub fn stage_orchestration(
    mut event: EventWriter<SpawnEnemyEvent>,
    mut stage_orchestration_state: ResMut<StageOrchestrationState>,
    time: Res<Time>,
) {
    if !stage_orchestration_state.grid.iter().any(|&x| x == false) {
        return;
    }
    stage_orchestration_state.next_spawn.tick(time.delta());
    if stage_orchestration_state.next_spawn.finished() {
        loop {
            let index = thread_rng().gen_range(0..16);
            if !stage_orchestration_state.grid[index] {
                let x = (((index % 4) as i32 * 125) - 250) as f32;
                let y = ((index / 4) as i32 * 96) as f32;
                event.send(SpawnEnemyEvent {
                    position: Vec2::new(x, y),
                    enemy_type: EnemyType::SmallSpider,
                });
                stage_orchestration_state.grid[index] = true;
                stage_orchestration_state.next_spawn.reset();
                break;
            }
        }
    }
}
