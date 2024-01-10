use bevy::{core_pipeline::clear_color::ClearColorConfig, render::camera::Viewport};

use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub spritesheet: SpriteSheetBundle,
    pub player: Player,
    pub health: Health,
    pub xp: Xp,
    pub size: Size,
    pub name: Name,

    pub noise_debug: PlayerNoiseDebug,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
    pub ccd: Ccd,

    pub collider: Collider,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
    pub enable_events: ActiveEvents,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::init("NoName"),
            health: Health::init(PLAYER_HEALTH, PLAYER_HEALTH),
            size: Size(Vec2::splat(PLAYER_SIZE)),
            xp: Xp(0),
            spritesheet: Default::default(),
            name: Name::new("Player"),

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
            mass: ColliderMassProperties::Density(10.0),
            restitution: Restitution::coefficient(0.5),
            enable_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_type: EnemyType,
    pub objective: EnemyObjective,
    pub change_state_timer: ChangeStateTimer,
    pub unchangable_timer: UnchangableTimer,
    pub follow_timer: FollowTimer,

    pub health: Health,
    pub size: Size,
    pub spritesheet_bundle: SpriteSheetBundle,
    pub name: Name,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub damping: Damping,

    pub collider: Collider,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            enemy_type: EnemyType::default(),
            objective: EnemyObjective::default(),
            change_state_timer: ChangeStateTimer::default(),
            unchangable_timer: UnchangableTimer::default(),
            follow_timer: FollowTimer::default(),
            health: Health::init(50, 50),
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
                angular_damping: 1.0,
            },
            collider: Collider::ball(ENEMY_SIZE / 2.0),
            mass: ColliderMassProperties::Density(0.1),
            restitution: Restitution::coefficient(0.75),
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub size: Size,
    pub sprite: SpriteBundle,
    pub name: Name,
    pub rbd: RigidBody,
    pub collider: Collider,
}
impl WallBundle {
    pub fn new(location: WallLocation) -> Self {
        let size = location.size();
        WallBundle {
            wall: Wall,
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
        }
    }
}

#[derive(Bundle)]
pub struct MainCamBundle {
    pub camera_bundle: Camera2dBundle,
    pub camera: MainCam,
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
                    scale: 1.,
                    ..default()
                },

                ..default()
            },
            camera: MainCam,
            name: "MainCam".into(),
        }
    }
}

#[derive(Bundle)]
pub struct MinimapCamBundle {
    pub camera_bundle: Camera2dBundle,
    pub camera: MinimapCam,
    pub name: Name,
}

impl Default for MinimapCamBundle {
    fn default() -> Self {
        let (size_x, size_y) = (300, 200);
        Self {
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
            camera: MinimapCam,
            name: "MinimapCam".into(),
        }
    }
}

#[derive(Bundle)]
pub struct HealthBarBundle {
    health_sprite: SpriteBundle,
    marker: HealthBar,
}
impl HealthBarBundle {
    pub fn new(color: Color, size: Vec2) -> Self {
        HealthBarBundle {
            health_sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_xyz(0., 17.5, 1.),
                ..default()
            },
            marker: HealthBar,
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
