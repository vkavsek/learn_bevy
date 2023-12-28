use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Size(pub Vec2);

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
impl Health {
    pub fn init(current: i32, max: i32) -> Self {
        Health { current, max }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Xp(pub usize);

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Collider;

#[derive(Bundle)]
pub struct HouseBundle {
    pub house: House,
    pub coll: Collider,
    pub size: Size,
    pub spritesheet: SpriteSheetBundle,
}
impl Default for HouseBundle {
    fn default() -> Self {
        Self {
            house: House,
            coll: Collider,
            size: Size(Vec2::splat(MAX_HOUSE_SIZE)),
            spritesheet: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub coll: Collider,
    pub size: Size,
    pub spritesheet: SpriteSheetBundle,
}
impl WallBundle {
    pub fn new(location: WallLocation, texture_atlas: Handle<TextureAtlas>) -> Self {
        let size = location.size();
        WallBundle {
            wall: Wall,
            coll: Collider,
            size: Size(size),
            spritesheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::RED,
                    custom_size: Some(size),
                    index: 10,
                    ..default()
                },

                transform: Transform {
                    translation: location.position().extend(1.0),
                    ..default()
                },
                texture_atlas,
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub enum WallLocation {
    Left,
    Right,
    Top,
    Bot,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        let start = -(MAP_SIZE as f32 * TILE_SIZE / 2.);
        let end = MAP_SIZE as f32 * TILE_SIZE / 2.;
        match self {
            WallLocation::Left => Vec2::new(start, 0.),
            WallLocation::Right => Vec2::new(end, 0.),
            WallLocation::Bot => Vec2::new(0., start),
            WallLocation::Top => Vec2::new(0., end),
        }
    }
    fn size(&self) -> Vec2 {
        let length = MAP_SIZE as f32 * TILE_SIZE;
        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(OUTSIDE_WALL_THICK, length),
            WallLocation::Bot | WallLocation::Top => Vec2::new(length, OUTSIDE_WALL_THICK),
        }
    }
}
