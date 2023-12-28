use noise::utils::NoiseMap;

use crate::prelude::*;

#[derive(Resource, Deref)]
pub struct Score(pub usize);

#[derive(Resource, Deref)]
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Deref)]
pub struct NoiseMapped(pub NoiseMap);

#[derive(Resource, Deref)]
pub struct MapRootHandle(pub Entity);
