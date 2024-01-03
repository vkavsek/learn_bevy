use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_toggle_active,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (create_map, load_spritesheet_texture))
            .add_systems(
                OnEnter(SetupState::Setup),
                (setup_player, setup_enemies, setup_game_cameras),
            )
            .add_systems(
                OnEnter(SetupState::Build),
                (generate_world, build_outside_walls),
            )
            .add_systems(OnExit(SetupState::Ready), map_world_cleanup);
    }
}

pub struct MainLogicPlugin;
impl Plugin for MainLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                (
                    // TODO: change everything to physics
                    handle_kbd_inputs,
                    handle_mouse_input,
                    handle_player_enemy_collisions,
                    tick_enemy_timers,
                    handle_enemy_objective_timers,
                    dynamic_damping,
                    cam_movement.after(handle_kbd_inputs),
                )
                    .run_if(in_state(SetupState::Ready)),
            ),
        );
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((
                WorldInspectorPlugin::new().run_if(input_toggle_active(true, KeyCode::P)),
                RapierDebugRenderPlugin::default(),
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin,
            ))
            .register_type::<Size>();
        }
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.),))
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..Default::default()
            });
    }
}
