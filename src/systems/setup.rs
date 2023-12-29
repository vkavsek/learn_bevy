pub mod map;
pub mod player_enemies;

pub mod general {
    use crate::prelude::*;
    pub fn setup_camera(mut cmds: Commands) {
        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 1.5;
        cmds.spawn(camera);
    }
}
