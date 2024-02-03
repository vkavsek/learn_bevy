use bevy::{core_pipeline::clear_color::ClearColorConfig, render::camera::Viewport};

use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub _gs: OnGameScreen,
    pub player: Player,
    pub spritesheet: SpriteSheetBundle,
    pub health: Health,
    pub xp: Xp,
    pub size: Size,
    pub name: Name,
    pub current_gun_type: GunType,

    pub noise_debug: PlayerNoiseDebug,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
    pub ccd: Ccd,

    pub collider: Collider,
    pub coll_groups: CollisionGroups,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
    pub enable_events: ActiveEvents,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            _gs: OnGameScreen,
            player: Player::init("NoName"),
            health: Health::init(PLAYER_HEALTH, PLAYER_HEALTH),
            size: Size(Vec2::splat(PLAYER_SIZE)),
            xp: Xp(0),
            spritesheet: Default::default(),
            name: Name::new("Player"),
            current_gun_type: GunType::default(),

            noise_debug: PlayerNoiseDebug(None),

            rbd: RigidBody::Dynamic,
            vel: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
            damping: Damping {
                linear_damping: PLAYER_DAMPING,
                angular_damping: 10.,
            },
            ccd: Ccd::enabled(),
            collider: Collider::ball(PLAYER_SIZE / 2.),
            coll_groups: CollisionGroups::new(
                Group::from_bits(0b1110).unwrap(),
                Group::from_bits(0b1111).unwrap(),
            ),
            mass: ColliderMassProperties::Density(10.0),
            restitution: Restitution::coefficient(0.5),
            enable_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub _gs: OnGameScreen,
    pub _b: Bullet,
    pub sprite: SpriteSheetBundle,
    pub name: Name,
    pub target: BulletTarget,
    pub bullet_life: BulletLifeTimer,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub damping: Damping,
    pub ccd: Ccd,

    pub collider: Collider,
    pub coll_groups: CollisionGroups,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
    pub enable_events: ActiveEvents,
}
impl BulletBundle {
    pub fn new(
        size: Vec2,
        char_texture: Handle<TextureAtlas>,
        translation: Vec2,
        rotation: Quat,
        target: Vec2,
        bullet_speed: f32,
    ) -> Self {
        let half_x = size.x / 2.;
        let half_y = size.y / 2.;

        let v = target * bullet_speed;

        Self {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::WHITE,
                    index: 30,
                    custom_size: Some(size),
                    ..default()
                },
                texture_atlas: char_texture,
                transform: Transform::from_xyz(translation.x, translation.y, 1.)
                    .with_rotation(rotation),
                ..default()
            },
            target: BulletTarget(target),
            vel: Velocity {
                linvel: v,
                angvel: 0.,
            },
            collider: Collider::convex_hull(&[
                Vec2::new(-half_x, -half_y),
                Vec2::new(half_x, -half_y),
                Vec2::new(0., half_y),
            ])
            .expect("Error computing convex hull for Bullets"),
            ..default()
        }
    }
}
impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            _gs: OnGameScreen,
            sprite: SpriteSheetBundle { ..default() },
            _b: Bullet,
            bullet_life: BulletLifeTimer::new(BULLET_LIFE),
            name: Name::new("Bullet"),
            rbd: RigidBody::Dynamic,
            target: BulletTarget(Vec2::ZERO),
            vel: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 10.,
            },
            ccd: Ccd::enabled(),
            collider: Collider::convex_hull(&[
                Vec2::new(-10., -10.),
                Vec2::new(10., -10.),
                Vec2::new(0., 10.),
            ])
            .expect("Error computing convex hull for Bullets"),
            coll_groups: CollisionGroups::new(
                Group::from_bits(0b0001).unwrap(),
                Group::from_bits(0b1110).unwrap(),
            ),
            mass: ColliderMassProperties::Density(1.0),
            restitution: Restitution::coefficient(0.8),
            enable_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub _gs: OnGameScreen,
    pub _e: Enemy,
    pub enemy_type: EnemyType,
    pub objective: EnemyObjective,
    pub change_state_timer: ChangeStateTimer,
    pub unchangable_timer: UnchangableTimer,
    pub shot_timer: EnemyShotTimer,

    pub health: Health,
    pub size: Size,
    pub spritesheet_bundle: SpriteSheetBundle,
    pub name: Name,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub damping: Damping,

    pub collider: Collider,
    pub coll_groups: CollisionGroups,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            _gs: OnGameScreen,
            _e: Enemy,
            enemy_type: EnemyType::default(),
            objective: EnemyObjective::default(),
            change_state_timer: ChangeStateTimer::default(),
            unchangable_timer: UnchangableTimer::default(),
            shot_timer: EnemyShotTimer::default(),
            health: Health::init(ENEMY_HEALTH, ENEMY_HEALTH),
            size: Size(Vec2::splat(ENEMY_SIZE)),
            spritesheet_bundle: Default::default(),
            name: Name::new("Enemy"),
            rbd: RigidBody::Dynamic,
            vel: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            damping: Damping {
                linear_damping: ENEMY_DAMPING,
                angular_damping: 10.0,
            },
            collider: Collider::ball(ENEMY_SIZE / 2.0),
            coll_groups: CollisionGroups::new(
                Group::from_bits(0b1110).unwrap(),
                Group::from_bits(0b1111).unwrap(),
            ),
            mass: ColliderMassProperties::Density(0.1),
            restitution: Restitution::coefficient(0.75),
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub _gs: OnGameScreen,
    pub _w: Wall,
    pub size: Size,
    pub sprite: SpriteBundle,
    pub name: Name,
    pub rbd: RigidBody,
    pub collider: Collider,
    pub coll_groups: CollisionGroups,
}
impl WallBundle {
    pub fn new(location: WallLocation) -> Self {
        let size = location.size();
        WallBundle {
            _gs: OnGameScreen,
            _w: Wall,
            size: Size(size),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: location.position().extend(1.0),
                    ..default()
                },
                ..default()
            },
            name: Name::new("Wall"),
            rbd: RigidBody::Fixed,
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            coll_groups: CollisionGroups::new(
                Group::from_bits(0b1110).unwrap(),
                Group::from_bits(0b1111).unwrap(),
            ),
        }
    }
}

#[derive(Bundle)]
pub struct MainCamBundle {
    pub camera_bundle: Camera2dBundle,
    pub _c: MainCam,
    pub name: Name,
}

impl Default for MainCamBundle {
    fn default() -> Self {
        Self {
            camera_bundle: Camera2dBundle {
                camera: Camera {
                    order: 0,
                    ..default()
                },
                projection: OrthographicProjection {
                    near: -1.0,
                    scale: 0.5,
                    ..default()
                },

                ..default()
            },
            _c: MainCam,
            name: "MainCam".into(),
        }
    }
}

#[derive(Bundle)]
pub struct MinimapCamBundle {
    pub _gs: OnGameScreen,
    pub camera_bundle: Camera2dBundle,
    pub _c: MinimapCam,
    pub name: Name,
}

impl Default for MinimapCamBundle {
    fn default() -> Self {
        let (size_x, size_y) = (300, 200);
        Self {
            _gs: OnGameScreen,
            camera_bundle: Camera2dBundle {
                camera: Camera {
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(25, WINDOW_RES.y as u32 - 15),
                        physical_size: UVec2::new(size_x, size_y),
                        depth: 0.0..0.1,
                    }),
                    order: 1,
                    ..default()
                },
                projection: OrthographicProjection {
                    near: -1.0,
                    scale: 25.,
                    ..default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                },
                ..default()
            },
            _c: MinimapCam,
            name: "MinimapCam".into(),
        }
    }
}

#[derive(Bundle)]
pub struct HealthBarBundle {
    pub _gs: OnGameScreen,
    health_sprite: SpriteBundle,
    _hb: HealthBar,
}
impl HealthBarBundle {
    pub fn new(color: Color, size: Vec2) -> Self {
        HealthBarBundle {
            _gs: OnGameScreen,
            health_sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_xyz(0., 17.5, 1.),
                ..default()
            },
            _hb: HealthBar { init_width: size.x },
        }
    }
}

#[derive(Bundle)]
pub struct GameCursorBundle {
    _gc: GameCursor,
    image_b: ImageBundle,
    name: Name,
}
impl GameCursorBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        GameCursorBundle {
            _gc: GameCursor,
            image_b: ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage {
                    texture,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            },
            name: "GameCursor".into(),
        }
    }
}
// #[derive(Bundle)]
// pub struct HouseBundle {
//     pub house: House,
//     pub size: Size,
//     pub spritesheet: SpriteSheetBundle,
//     pub name: Name,
//
//     pub rbd: RigidBody,
//     pub collider: Collider,
// }
// impl Default for HouseBundle {
//     fn default() -> Self {
//         Self {
//             house: House,
//             size: Size(Vec2::splat(MAX_HOUSE_SIZE)),
//             spritesheet: Default::default(),
//             name: Name::new("House"),
//             rbd: RigidBody::Fixed,
//             collider: Collider::cuboid(MAX_HOUSE_SIZE / 2., MAX_HOUSE_SIZE / 2.),
//         }
//     }
// }
