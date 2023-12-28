use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, create_map)
            .add_systems(OnEnter(MapState::Build), (generate_world, build_houses))
            .add_systems(OnExit(MapState::Ready), map_world_cleanup);
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_spritesheet_texture)
            .add_systems(
                OnEnter(MapState::Setup),
                (setup_player, setup_enemies, setup_camera),
            )
            .add_systems(
                Update,
                (
                    bevy::window::close_on_esc,
                    (
                        handle_input,
                        apply_velocity,
                        (check_for_collisions_player, cam_movement).after(apply_velocity),
                    )
                        .run_if(in_state(MapState::Ready)),
                ),
            );
    }
}
