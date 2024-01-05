use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(
                OnEnter(SetupState::Setup),
                (setup_fps_counter, setup_debug_text),
            )
            .add_systems(
                Update,
                (handle_fps_update, handle_debug_text, fps_visibility),
            )
            .register_type::<Size>()
            .register_type::<EnemyObjective>()
            .register_type::<ChangeStateTimer>()
            .register_type::<UnchangableTimer>()
            .register_type::<FollowTimer>()
            .register_type::<Health>()
            .register_type::<Xp>();
        }
    }
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_spritesheet_texture)
            .add_systems(
                OnEnter(SetupState::Setup),
                (setup_player, setup_enemies, setup_game_cameras),
            );
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
                    (dynamic_damping, cam_movement).after(handle_kbd_inputs),
                )
                    .run_if(in_state(SetupState::Ready)),
            ),
        );
    }
}
pub struct EnemyLogicPlugin;
impl Plugin for EnemyLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_enemy_color,
                handle_enemy_timers,
                enemy_follow_player,
                handle_player_enemy_collisions.after(handle_enemy_timers),
            )
                .run_if(in_state(SetupState::Ready)),
        );
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

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin).add_systems(
            OnEnter(SetupState::Build),
            (generate_tilemap, build_outside_walls),
        );
    }
}
