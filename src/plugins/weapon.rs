use bevy::prelude::*;
use crate::plugins::cooldown::Cooldown;
use crate::plugins::projectile::*;

#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub projectile_type: ProjectileType,
    pub cooldown: Cooldown,
}

impl Weapon {
    pub fn new(projectile_type: ProjectileType, fire_rate: f32) -> Self {
        Self {
            projectile_type,
            cooldown: Cooldown::new(1.0 / fire_rate),
        }
    }
}

pub fn shoot_weapon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    direction: Vec3,
    weapon: &mut Weapon
) {
    if weapon.cooldown.is_ready() {
        spawn_projectile(
            &mut commands,
            &mut meshes,
            &mut materials,
            weapon.projectile_type.clone(),
            position,
            direction,
        );
        weapon.cooldown.reset()
    }
}
