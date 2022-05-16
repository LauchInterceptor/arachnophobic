mod orchestration;
mod waves;

use crate::prelude::*;

use self::orchestration::*;

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
