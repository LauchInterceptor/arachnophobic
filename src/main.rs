mod player;

use bevy::core::FixedTimestep;
use bevy::math::const_vec2;
use bevy::prelude::*;
use player::*;

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const WINDOW_SIZE: Vec2 = const_vec2!([600.0, 800.0]);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "ARACHNOPHOBIC!".to_string(),
            width: 600.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
    println!("Exiting");
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    // Create Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Windows Position
    let window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(500, 500));
    // Spawn sprite
    spawn_player(&mut commands);
}
