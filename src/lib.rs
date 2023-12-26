pub mod prelude {
    pub use crate::*;
    pub use crate::{comps_n_resources::*, systems::*};
    pub use bevy::prelude::*;

    pub const TITLE: &str = "Game";
    pub const BG_COLOR: Color = Color::rgb(0.0, 0.4, 0.2);

    pub const WINDOW_RES: (f32, f32) = (1280., 900.);
    pub const HALF_WIDTH: f32 = WINDOW_RES.0 / 2.0;
    pub const HALF_HEIGHT: f32 = WINDOW_RES.1 / 2.0;

    pub const PLAYER_SPEED: f32 = 300.0;
    pub const PLAYER_SPRITE_WIDTH: f32 = 100.0;
    pub const PLAYER_SPRITE_HEIGHT: f32 = 125.0;
    pub const ENEMY_SPEED: f32 = 100.0;
    pub const ENEMY_SPRITE_WIDTH: f32 = 75.0;
}

mod comps_n_resources;
mod systems;

use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                handle_input,
                player_movement,
                enemy_movement,
            ),
        );
    }
}
