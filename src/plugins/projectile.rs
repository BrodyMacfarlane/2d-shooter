use bevy::prelude::*;
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

#[derive(Component)]
pub struct Projectile;

#[derive(Component, Debug, Clone)]
pub enum ProjectileType {
    Bullet,
    Arrow,
}

#[derive(Component, Debug)]
pub struct ProjectileProperties {
    pub speed: f32,
    pub radius: f32,
    pub travelled_distance: f32,
    pub max_distance: f32,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub projectile_type: ProjectileType,
    pub properties: ProjectileProperties,
    pub velocity: Velocity,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, handle_projectile_despawn));
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform, &mut ProjectileProperties)>, time: Res<Time>) {
    for (velocity, mut transform, mut properties) in query.iter_mut() {
        let movement = velocity.value * time.delta_seconds();
        transform.translation += movement;
        properties.travelled_distance += movement.length();
    }
}

fn handle_projectile_despawn(
    mut commands: Commands,
    query: Query<(Entity, &ProjectileProperties), With<Projectile>>
) {
    for (entity, properties) in query.iter() {
        if properties.travelled_distance > properties.max_distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    projectile_type: ProjectileType,
    position: Vec3,
    direction: Vec3,
) {
    let (speed, radius, max_distance, color) = match projectile_type {
        ProjectileType::Bullet => (1250.0, 2.0, 1000.0, Color::srgb(0.9, 0.0, 0.9)),
        ProjectileType::Arrow => (350.0, 5.0, 200.0, Color::srgb(0.0, 0.9, 0.9)),
    };

    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        projectile_type: projectile_type.clone(),
        properties: ProjectileProperties { 
            speed, 
            radius, 
            travelled_distance: 0.0, 
            max_distance 
        },
        velocity: Velocity::new(direction.normalize() * speed),
        mesh: MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(radius)).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(position),
            ..default()
        },
    });
}
