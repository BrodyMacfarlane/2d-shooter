mod components;
mod systems;
mod resources;

//pub use components::*;
pub use systems::*;
pub use resources::*;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites)
           .init_resource::<AnimationAssets>();
    }
}
