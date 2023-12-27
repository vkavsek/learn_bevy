use crate::prelude::*;

#[derive(Resource, Deref)]
pub struct Score(pub usize);

#[derive(Resource, Deref)]
pub struct CharSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Deref)]
pub struct MapSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Deref)]
pub struct NoiseMapRoot(pub Entity);
