mod plugins;

use bevy::prelude::*;

use plugins::scene::ScenePlugin;
use plugins::player::PlayerPlugin;
use plugins::animation::AnimationPlugin;
use plugins::camera::CameraPlugin;
use plugins::projectile::ProjectilePlugin;
use plugins::cooldown::CooldownPlugin;


fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::srgb(0.12, 0., 0.25)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(ScenePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(CooldownPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
