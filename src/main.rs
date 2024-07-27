mod plugins;

use bevy::prelude::*;

use plugins::scene::ScenePlugin;
//use plugins::movement::MovementPlugin;
use plugins::player::PlayerPlugin;
use plugins::camera::CameraPlugin;
//use plugins::collision::CollisionPlugin;


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
        //.add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        //.add_plugins(CollisionPlugin)
        // .add_plugins(DebugPlugin)
        //
        // .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        // .add_systems(Update, (move_player, update_camera).chain())
        .run();
}
