use crate::prelude::*;
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    BasicMulti, Perlin,
};
use rand::{thread_rng, Rng};

pub fn map_world_cleanup(mut commands: Commands, noise_map: Res<MapRootHandle>) {
    commands.entity(**noise_map).despawn_recursive();
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

fn generate_noise_map() -> NoiseMap {
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    let mut bm = BasicMulti::<Perlin>::new(seed);
    bm.lacunarity = 1.;
    bm.frequency = 2.;

    PlaneMapBuilder::<_, 2>::new(&bm)
        .set_size(MAP_SIZE, MAP_SIZE)
        .build()
}

fn get_color(val: f64) -> Color {
    let color_result = match val.abs() {
        v if v < 0.1 => Color::hex("#00ff00"),
        v if v < 0.2 => Color::hex("#3ff03f"),
        v if v < 0.25 => Color::hex("b35900"),
        v if v < 0.3 => Color::hex("#ffff1a"),
        v if v <= 1.0 => Color::hex("#8080ff"),
        _ => panic!("unexpected value"),
    };
    color_result.expect("Getting color from HEX error")
}
fn get_index(val: f64) -> usize {
    match val.abs() {
        v if v < 0.20 => 60,
        v if v < 0.25 => 0,
        v if v < 0.3 => 61,
        v if v <= 1.0 => 247,
        _ => panic!("unexpected value"),
    }
}

pub fn create_map(mut commands: Commands) {
    let map = generate_noise_map();
    commands.insert_resource(NoiseMapped(map));
}

pub fn build_houses(
    mut commands: Commands,
    map: Res<NoiseMapped>,
    map_texture: Res<AsciiSpriteSheet>,
) {
    let mut rng = thread_rng();
    let mut house_positions = Vec::with_capacity(NUM_OF_HOUSES);
    let (map_w, map_h) = map.size();

    let start_x = -(map_w as f32) * TILE_SIZE / 2.;
    let start_y = -(map_h as f32) * TILE_SIZE / 2.;

    let house_s = (MAX_HOUSE_SIZE / TILE_SIZE) as usize;
    while house_positions.len() < NUM_OF_HOUSES {
        let rand_x = rng.gen_range(house_s..map_w - house_s);
        let rand_y = rng.gen_range(house_s..map_h - house_s);
        let x = start_x + rand_x as f32 * TILE_SIZE;
        let y = start_y + rand_y as f32 * TILE_SIZE;
        let size = rng.gen_range(150_f32..MAX_HOUSE_SIZE);

        let val = map.get_value(rand_x, rand_y);
        if val.abs() < 0.1 {
            house_positions.push((x, y, size));
        }
    }

    for (x, y, size) in house_positions {
        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 10,
                color: Color::hex("#9b1c00").unwrap(),
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            texture_atlas: map_texture.0.clone(),
            transform: Transform::from_translation(Vec3::new(x, y, 50.)),
            ..Default::default()
        });
    }
}

pub fn generate_world(
    mut commands: Commands,
    map_texture: Res<AsciiSpriteSheet>,
    map: Res<NoiseMapped>,
    mut next_state: ResMut<NextState<MapState>>,
) {
    let (map_w, map_h) = map.size();
    info!("Map size: {map_w}x{map_h}");

    let start_x = -(map_w as f32) * TILE_SIZE / 2.;
    let start_y = -(map_h as f32) * TILE_SIZE / 2.;

    let noise_map_root = commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            for x_pos in 0..map_w {
                for y_pos in 0..map_h {
                    let val = map.get_value(x_pos, y_pos);
                    let x = start_x + x_pos as f32 * TILE_SIZE;
                    let y = start_y + y_pos as f32 * TILE_SIZE;

                    parent.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: get_index(val),
                            color: get_color(val),
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..Default::default()
                        },
                        texture_atlas: map_texture.0.clone(),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..Default::default()
                    });
                }
            }
        })
        .id();
    commands.insert_resource(MapRootHandle(noise_map_root));

    next_state.set(MapState::Setup);
}
