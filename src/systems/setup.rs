pub mod map;
pub mod player_enemies;

pub mod general {
    use crate::prelude::*;

    pub fn setup_game_cameras(mut cmds: Commands) {
        cmds.spawn(MainCamBundle::default());
        cmds.spawn(MinimapCamBundle::default());
    }

    pub fn load_spritesheet_texture(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    ) {
        let image = asset_server.load::<Image>("tileset-16x16.png");
        let atlas = TextureAtlas::from_grid(image, Vec2::splat(16.0), 16, 16, None, None);
        let atlas_handle = texture_atlas.add(atlas);
        commands.insert_resource(AsciiSpriteSheet(atlas_handle));
    }

    pub fn map_world_cleanup(mut commands: Commands, noise_map: Res<MapRootHandle>) {
        commands.entity(**noise_map).despawn_recursive();
    }
}
