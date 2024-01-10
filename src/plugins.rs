use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

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

pub struct EnemyLogicPlugin;
impl Plugin for EnemyLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_enemy,
                handle_enemy_timers,
                (
                    change_enemy_color,
                    enemy_follow_player,
                    handle_enemy_player_coll,
                )
                    .after(handle_enemy_timers),
            )
                .run_if(in_state(SetupState::Ready)),
        );
    }
}

pub struct MainLogicPlugin;
impl Plugin for MainLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(PreStartup, load_spritesheet_texture)
            .add_systems(
                OnEnter(SetupState::Build),
                (generate_tilemap, build_outside_walls),
            )
            .add_systems(
                OnEnter(SetupState::Setup),
                (setup_player, setup_enemies, setup_game_cameras),
            )
            .add_systems(
                FixedUpdate,
                (handle_kbd_inputs).run_if(in_state(SetupState::Ready)),
            )
            .add_systems(
                Update,
                (
                    bevy::window::close_on_esc,
                    (
                        handle_mouse_input,
                        dynamic_damping,
                        handle_healthbars,
                        toggle_healthbar_vis,
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
        let rdr = RapierDebugRenderPlugin::default();
        app.add_plugins((
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
            RapierDebugRenderPlugin::disabled(rdr),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(
            OnEnter(SetupState::Setup),
            (setup_fps_counter, setup_debug_text),
        )
        .add_systems(
            Update,
            (
                handle_fps_update,
                update_debug_text,
                toggle_debug_info_vis,
                toggle_rapier_debug_render,
            ),
        )
        .register_type::<Size>()
        .register_type::<EnemyObjective>()
        .register_type::<ChangeStateTimer>()
        .register_type::<PlayerNoiseDebug>()
        .register_type::<UnchangableTimer>()
        .register_type::<FollowTimer>()
        .register_type::<HealthBar>()
        .register_type::<Health>()
        .register_type::<Xp>();
    }
}
