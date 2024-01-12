use crate::prelude::*;

// #[derive(Resource, Deref)]
// pub struct Score(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut)]
pub struct MapGenSeed(pub u32);

#[derive(Resource, Deref, DerefMut, Reflect, Default)]
#[reflect(Resource)]
pub struct BulletSpawnTimer(pub Timer);
impl BulletSpawnTimer {
    pub fn new() -> Self {
        BulletSpawnTimer(Timer::new(BULLET_SPAWN_INTERVAL, TimerMode::Once))
    }
}
