mod assets;
mod components;
mod enemy;
mod menu;
mod player;
mod projectile;
mod state;
mod util;
mod weapons;

use bevy::math::const_vec2;
use bevy_asset_loader::AssetCollectionApp;

mod prelude {
    pub use crate::assets::*;
    pub use crate::components::*;
    pub use crate::enemy::*;
    pub use crate::menu::*;
    pub use crate::player::*;
    pub use crate::projectile::*;
    pub use crate::state::{AppState, GameState::*, MenuState::*};
    pub use crate::util::*;
    pub use crate::weapons::*;
    pub use bevy::prelude::*;
    pub use heron::prelude::*;
    pub use iyes_loopless::prelude::*;
}

use crate::prelude::*;
pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const WINDOW_SIZE: Vec2 = const_vec2!([600.0, 800.0]);

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "ARACHNOPHOBIC!".to_string(),
        width: 600.0,
        height: 800.0,
        resizable: false,
        ..Default::default()
    })
    // Intial State
    .add_loopless_state(AppState::Menu(StartMenu))
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_event::<SpawnProjectileEvent>()
    .add_event::<SpawnEnemyEvent>()
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin::default())
    .init_collection::<TextureAtlasAssets>()
    .init_collection::<SpriteAssets>()
    .init_collection::<FontAssets>()
    .init_resource::<StageOrchestrationState>()
    .add_startup_system(setup)
    .add_plugin(MenuPlugin)
    .add_enter_system(AppState::Game(InGame), spawn_player)
    .add_system_set(
        ConditionSet::new()
            .run_in_state(AppState::Game(InGame))
            .with_system(player_movement)
            .with_system(animate_player)
            .with_system(update_projectile)
            .with_system(player_shoot)
            .with_system(spawn_projectile)
            .with_system(spawn_enemy)
            .with_system(enemy_orchestration)
            .into(),
    )
    .add_system(bevy::input::system::exit_on_esc_system)
    .run();
    println!("Exiting");
}

fn setup(mut commands: Commands) {
    // Create Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
