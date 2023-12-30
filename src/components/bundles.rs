use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub spritesheet: SpriteSheetBundle,
    pub player: Player,
    pub health: Health,
    pub xp: Xp,
    pub size: Size,
    pub name: Name,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
    pub ccd: Ccd,

    pub collider: Collider,
    pub mass: ColliderMassProperties,
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
            rbd: RigidBody::Dynamic,
            vel: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
            damping: Damping {
                linear_damping: PLAYER_SPEED / 10.,
                angular_damping: PLAYER_SPEED / 10.,
            },
            ccd: Ccd::enabled(),
            collider: Collider::ball(PLAYER_SIZE / 2.),
            mass: ColliderMassProperties::Density(3.0),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_type: EnemyType,
    pub objective: EnemyObjective,

    pub health: Health,
    pub size: Size,
    pub spritesheet_bundle: SpriteSheetBundle,
    pub name: Name,

    pub rbd: RigidBody,
    pub vel: Velocity,
    pub damping: Damping,

    pub collider: Collider,
    pub mass: ColliderMassProperties,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            enemy_type: EnemyType::default(),
            objective: EnemyObjective::default(),
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
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            collider: Collider::ball(ENEMY_SIZE / 2.0),
            mass: ColliderMassProperties::Density(0.2),
        }
    }
}

#[derive(Bundle)]
pub struct HouseBundle {
    pub house: House,
    pub size: Size,
    pub spritesheet: SpriteSheetBundle,
    pub name: Name,
    pub rbd: RigidBody,
}
impl Default for HouseBundle {
    fn default() -> Self {
        Self {
            house: House,
            size: Size(Vec2::splat(MAX_HOUSE_SIZE)),
            spritesheet: Default::default(),
            name: Name::new("House"),
            rbd: RigidBody::Fixed,
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub size: Size,
    pub spritesheet: SpriteSheetBundle,
    pub name: Name,
    pub rbd: RigidBody,
}
impl WallBundle {
    pub fn new(location: WallLocation, texture_atlas: Handle<TextureAtlas>) -> Self {
        let size = location.size();
        WallBundle {
            wall: Wall,
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
            name: Name::new("Wall"),
            rbd: RigidBody::Fixed,
        }
    }
}
