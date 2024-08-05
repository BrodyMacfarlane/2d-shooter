use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

use crate::plugins::projectile::*;
use crate::plugins::weapon::shoot_weapon;
use crate::plugins::camera::MainCamera;

use super::scene::ENTITIES_Z;
use super::weapon::Weapon;

const PLAYER_Z_LAYER: f32 = ENTITIES_Z;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, PLAYER_Z_LAYER);
const PLAYER_RADIUS: f32 = 5.0;
const PLAYER_SPEED: f32 = 250.;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, (player_movement_controls, player_weapon_controls));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    
    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(PLAYER_RADIUS)).into(),
            material: materials.add(Color::srgb(6.25, 9.4, 9.1)),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
        Weapon::new(ProjectileType::Bullet, 0.5)
    ));
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


fn player_weapon_controls(
    commands: Commands,
    mut player_query: Query<(&Transform, &mut Weapon), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((player_transform, mut player_weapon)) = player_query.get_single_mut() else {
        return;
    };
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let player_position = player_transform.translation;
        let direction = (world_position - player_position.truncate()).extend(0.0).normalize();

        if mouse_input.pressed(MouseButton::Left) {
            shoot_weapon(
                commands,
                meshes,
                materials,
                player_position,
                direction,
                &mut player_weapon,
            );
        }
    }
}
