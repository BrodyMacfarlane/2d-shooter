use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationAssets {
    pub player_spritesheet: Handle<TextureAtlas>,
}

impl Default for AnimationAssets {
    fn default() -> Self {
        AnimationAssets {
            player_spritesheet: Handle::default(),
        }
    }
}
