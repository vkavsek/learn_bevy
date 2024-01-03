use bevy::input::common_conditions::input_toggle_active;
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
                tick_enemy_timers,
                enemy_follow_player,
                handle_enemy_objective_timers,
            )
                .run_if(in_state(SetupState::Ready)),
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
                // TODO: FPS
                // LogDiagnosticsPlugin::default(),
                // FrameTimeDiagnosticsPlugin,
            ))
            .register_type::<Size>()
            .register_type::<EnemyObjective>()
            .register_type::<ChangeStateTimer>()
            .register_type::<FollowTimer>()
            .register_type::<Health>()
            .register_type::<Xp>();
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
