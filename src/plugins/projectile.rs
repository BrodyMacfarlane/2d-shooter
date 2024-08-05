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


#[derive(Bundle)]
pub struct ProjectileBundle {
    pub velocity: Velocity,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

pub const BULLET_SPEED: f32 = 100.;
pub const BULLET_RADIUS: f32 = 1.;

#[derive(Component)]
pub struct Bullet;

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
