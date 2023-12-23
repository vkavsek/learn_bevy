pub mod prelude {
    pub use crate::*;
    pub use crate::{comps_n_resources::*, systems::*};
    pub use bevy::prelude::*;
}

mod comps_n_resources;
mod systems;

use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
