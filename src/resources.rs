use crate::prelude::*;

// #[derive(Resource, Deref)]
// pub struct Score(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut)]
pub struct MapGenSeed(pub u32);
