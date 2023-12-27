use crate::prelude::*;
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    BasicMulti, Perlin,
};
use rand::{thread_rng, Rng};

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

pub fn generate_world(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let map = generate_noise_map();
    let (map_w, map_h) = map.size();

    info!("Map size: {map_w}x{map_h}");

    let tile_size = 32_f32;

    let start_x = -(map_w as f32) * tile_size / 2.;
    let start_y = -(map_h as f32) * tile_size / 2.;

    let noise_map_root = commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            for x_pos in 0..map_w {
                for y_pos in 0..map_h {
                    let val = map.get_value(x_pos, y_pos);
                    let x = start_x + x_pos as f32 * tile_size;
                    let y = start_y + y_pos as f32 * tile_size;

                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: get_color(val),
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..Default::default()
                    });
                }
            }
        })
        .id();
    commands.insert_resource(NoiseMapRoot(noise_map_root));

    next_state.set(AppState::Setup);
}
