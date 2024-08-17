use bevy::prelude::*;

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub const ENTITIES_Z: f32 = 2.;

pub struct ScenePlugin;

impl Plugin for ScenePlugin { 
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_scene, setup_instructions));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World where we move the player
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(5000., 2500.))),
        material: materials.add(Color::srgb(0.2, 0., 0.5)),
        ..default()
    });
}

fn setup_instructions(mut commands: Commands) {
    //commands.spawn(
    //    TextBundle::from_section(
    //        "Move the light with ZQSD or WASD.\nThe camera will smoothly track the light.",
    //        TextStyle::default(),
    //    )
    //    .with_style(Style {
    //        position_type: PositionType::Absolute,
    //        bottom: Val::Px(12.0),
    //        left: Val::Px(12.0),
    //        ..default()
    //    }),
    //);
}
