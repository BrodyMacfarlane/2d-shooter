use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Distance {
    pub travelled: f32,
    pub max: f32
}


#[derive(Bundle)]
pub struct ProjectileBundle {
    pub velocity: Velocity,
    pub distance: Distance,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, handle_despawn));
    }
}

pub const BULLET_SPEED: f32 = 100.;
pub const BULLET_RADIUS: f32 = 1.;
pub const BULLET_MAX_DISTANCE: f32 = 1000.;

#[derive(Component)]
pub struct Bullet;

fn update_position(mut query: Query<(&Velocity, &mut Transform, &mut Distance)>, time: Res<Time>) {
    for (velocity, mut transform, mut distance) in query.iter_mut() {
        let movement = velocity.value * time.delta_seconds();
        transform.translation += movement;
        distance.travelled += movement.length();
    }
}

fn handle_despawn(mut commands: Commands, query: Query<(Entity, &Distance)>) {
    for (entity, distance) in query.iter() {
        if distance.travelled > distance.max {
            commands.entity(entity).despawn_recursive();
        }
    }
}
