use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;

const PLAYER_Z_LAYER: f32 = 2.;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, PLAYER_Z_LAYER);
const PLAYER_RADIUS: f32 = 5.0;
const PLAYER_SPEED: f32 = 250.;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement_controls);
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
            material: materials.add(Color::srgb(6.25, 9.4, 9.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform {
                translation: STARTING_TRANSLATION,
                ..default()
            },
            ..default()
        },
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
