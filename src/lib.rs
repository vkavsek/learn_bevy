pub mod prelude {
    pub use crate::*;
    pub use crate::{comps_n_resources::*, systems::*};
    pub use bevy::prelude::*;

    pub const PLAYER_SPEED: f32 = 500.0;
    pub const BG_COLOR: Color = Color::rgb(0.0, 0.4, 0.2);
    // pub const WINDOW_SIZE: Vec2 = Vec2 { x: 1280., y: 960. };
}

mod comps_n_resources;
mod systems;

use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_systems(Startup, setup)
            .add_systems(Update, (bevy::window::close_on_esc, player_movement));
    }
}
