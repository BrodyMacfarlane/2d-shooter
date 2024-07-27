use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::vec3;

use crate::{
    //scene::SceneAssets,
    //collision::Collider,
    //movement::{Acceleration, MovingObjectBundle, Velocity},
};

const STARTING_TRANSLATION: Vec2 = Vec2::new(0.0, 0.0);
const PLAYER_RADIUS: f32 = 5.0;
const PLAYER_SPEED: f32 = 250.;
const BULLET_SPEED: f32 = 50.0;
const BULLET_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const BULLET_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Bullet;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement_controls);
            //.add_systems(Update, (player_movement_controls, player_weapon_controls));
    }
}

fn spawn_player(
    mut commands: Commands,
    //scene_assets: Res<SceneAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    
    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(15.)).into(),
            material: materials.add(Color::srgb(6.25, 9.4, 9.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform {
                translation: vec3(0., 0., 2.),
                ..default()
            },
            ..default()
        },
    ));

    //commands.spawn((
    //    MovingObjectBundle {
    //        velocity: Velocity::new(Vec2::ZERO),
    //        acceleration: Acceleration::new(Vec2::ZERO),
    //        collider: Collider::new(PLAYER_RADIUS),
    //        model: SceneBundle {
    //            scene: scene_assets.player.clone(),
    //            transform: Transform::from_translation(STARTING_TRANSLATION),
    //            ..default()
    //        },
    //    },
    //    Player,
    //));
}

fn player_movement_controls(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player.translation += move_delta.extend(0.);
}


//fn player_weapon_controls(
//    mut commands: Commands,
//    query: Query<&Transform, With<Player>>,
//    mouse_input: Res<ButtonInput<MouseButton>>,
//    scene_assets: Res<SceneAssets>,
//) {
//    let transform = query.single();
//    if mouse_input.pressed(MouseButton::Left) {
//        commands.spawn((
//            MovingObjectBundle {
//                velocity: Velocity::new(-transform.forward() * BULLET_SPEED),
//                acceleration: Acceleration::new(Vec2::ZERO),
//                collider: Collider::new(BULLET_RADIUS),
//                model: SceneBundle {
//                    scene: scene_assets.bullets.clone(),
//                    transform: Transform::from_translation(
//                        transform.translation + -transform.forward() * BULLET_FORWARD_SPAWN_SCALAR,
//                    ),
//                    ..default()
//                },
//            },
//            Bullet,
//        ));
//    }
//}
