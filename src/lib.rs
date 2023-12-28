pub mod prelude {
    pub use crate::*;
    pub use crate::{
        components::{common::*, enemy::*, player::*},
        resources::*,
        systems::{
            movement::*,
            setup::{general::*, map::*, player_enemies::*},
            *,
        },
    };
    pub use bevy::prelude::*;

    pub const TITLE: &str = "Game";
    pub const BG_COLOR: Color = Color::rgb(0., 0., 0.);

    pub const WINDOW_RES: (f32, f32) = (1280., 900.);

    pub const MAP_SIZE: usize = 200;
    pub const TILE_SIZE: f32 = 32_f32;
    pub const MAP_SIZE_PX: f32 = MAP_SIZE as f32 * TILE_SIZE;
    pub const NUM_OF_HOUSES: usize = 5;

    pub const PLAYER_SPEED: f32 = 300.0;
    pub const PLAYER_SPRITE_WIDTH: f32 = 32.0;
    pub const PLAYER_SPRITE_HEIGHT: f32 = 32.0;
    pub const ENEMY_SPEED: f32 = 100.0;
    pub const ENEMY_SPRITE_WIDTH: f32 = 32.0;
}

mod components;
mod resources;
mod systems;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Build,
    Setup,
    Ready,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (load_spritesheet_texture, create_map))
            .add_systems(OnEnter(AppState::Build), (generate_world, build_houses))
            .add_systems(
                OnEnter(AppState::Setup),
                (setup_player, setup_enemies, setup_camera),
            )
            .add_systems(
                Update,
                (
                    bevy::window::close_on_esc,
                    (
                        handle_input,
                        player_movement,
                        enemy_movement,
                        cam_movement.after(player_movement),
                    )
                        .run_if(in_state(AppState::Ready)),
                ),
            )
            .add_systems(OnExit(AppState::Ready), map_world_cleanup);
    }
}
