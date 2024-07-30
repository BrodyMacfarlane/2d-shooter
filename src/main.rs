mod plugins;

use bevy::prelude::*;

use plugins::scene::ScenePlugin;
use plugins::player::PlayerPlugin;
use plugins::camera::CameraPlugin;


fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::srgb(0.4, 0.2, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(ScenePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
