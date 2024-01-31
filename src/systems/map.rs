use crate::prelude::*;
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    BasicMulti, Perlin,
};
use rand::{thread_rng, Rng};

fn generate_noise_map(commands: &mut Commands) -> NoiseMap {
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    commands.insert_resource(MapGenSeed(seed));
    let mut bm = BasicMulti::<Perlin>::new(seed);
    bm.lacunarity = 1.;
    bm.frequency = 2.;

    PlaneMapBuilder::<_, 2>::new(&bm)
        .set_size(MAP_SIZE.x as usize, MAP_SIZE.y as usize)
        .build()
}

pub fn generate_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let noise_map = generate_noise_map(&mut commands);
    let map_size = MAP_SIZE;

    let mut tile_storage = TileStorage::empty(map_size);

    let tilemap_ent = commands.spawn_empty().id();

    let mut collected_tiles = Vec::with_capacity((map_size.x * map_size.y).try_into().unwrap());
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos::new(x, y);
            let noise_val = noise_map.get_value(x as usize, y as usize);
            let tile_ent = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_ent),
                        texture_index: TileTextureIndex(5),
                        color: TileColor(get_color(noise_val)),
                        ..Default::default()
                    },
                    GameMapTile::new(noise_val as f32),
                ))
                .id();
            tile_storage.set(&tile_pos, tile_ent);
            collected_tiles.push(tile_ent);
        }
    }

    let texture_handle = asset_server.load("tiles.png");
    let tile_size = TILE_SIZE;
    let grid_size = GRID_SIZE;
    let map_type = TilemapType::default();

    commands
        .entity(tilemap_ent)
        .insert((
            TilemapBundle {
                grid_size,
                map_type,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle),
                tile_size,
                transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
                ..Default::default()
            },
            Name::new("TilemapPARENT"),
        ))
        .push_children(&collected_tiles);

    next_state.set(AppState::Setup)
}

fn get_color(val: f64) -> Color {
    let saturation = 0.5;
    let color_result = match val.abs() {
        v if v < 0.1 => Color::hex("#00ff00").map(|col| col.with_s(saturation)),
        v if v < 0.2 => Color::hex("#3ff03f").map(|col| col.with_s(saturation)),
        v if v < 0.25 => Color::hex("b35900").map(|col| col.with_s(saturation)),
        v if v < 0.3 => Color::hex("#ffff1a").map(|col| col.with_s(saturation)),
        v if v <= 1.0 => Color::hex("#8080ff").map(|col| col.with_s(saturation)),
        _ => panic!("unexpected value"),
    };
    color_result.expect("Getting color from HEX error")
}

pub fn build_outside_walls(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bot));
}

// pub fn build_houses(
//     mut commands: Commands,
//     map: Res<NoiseMapValues>,
//     map_texture: Res<AsciiSpriteSheet>,
// ) {
//     let mut rng = thread_rng();
//     let mut house_positions = Vec::with_capacity(NUM_OF_HOUSES);
//     let (map_w, map_h) = map.size();
//
//     let start_x = -(map_w as f32) * TILE_SIZE / 2.;
//     let start_y = -(map_h as f32) * TILE_SIZE / 2.;
//
//     let house_s = (MAX_HOUSE_SIZE / TILE_SIZE) as usize;
//     while house_positions.len() < NUM_OF_HOUSES {
//         let rand_x = rng.gen_range(house_s..map_w - house_s);
//         let rand_y = rng.gen_range(house_s..map_h - house_s);
//         let x = start_x + rand_x as f32 * TILE_SIZE;
//         let y = start_y + rand_y as f32 * TILE_SIZE;
//         let size = rng.gen_range(150_f32..MAX_HOUSE_SIZE);
//
//         let val = map.get_value(rand_x, rand_y);
//         if val.abs() < 0.1 {
//             house_positions.push((x, y, size));
//         }
//     }
//
//     for (x, y, size) in house_positions {
//         commands.spawn(HouseBundle {
//             spritesheet: SpriteSheetBundle {
//                 sprite: TextureAtlasSprite {
//                     index: 255,
//                     color: Color::hex("#9b1c00").unwrap(),
//                     custom_size: Some(Vec2::splat(size)),
//                     ..Default::default()
//                 },
//                 texture_atlas: map_texture.clone(),
//                 transform: Transform::from_translation(Vec3::new(x, y, 0.1)),
//                 ..Default::default()
//             },
//             size: Size(Vec2::splat(size)),
//             collider: Collider::cuboid(size / 2.0, size / 2.0),
//             ..default()
//         });
//     }
// }
