use crate::prelude::*;

// #[derive(Resource, Deref)]
// pub struct Score(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut)]
pub struct MapGenSeed(pub u32);

#[derive(Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct BulletSpawnTimer(pub Timer);
impl BulletSpawnTimer {
    pub fn new(gun_type: &GunType) -> Self {
        BulletSpawnTimer(Timer::new(gun_type.type_to_interval(), TimerMode::Once))
    }
}
impl Default for BulletSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(BULLET_SPAWN_INTERVAL, TimerMode::Once))
    }
}
