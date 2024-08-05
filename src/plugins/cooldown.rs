use bevy::prelude::*;

use crate::plugins::weapon::Weapon;

pub trait HasCooldown {
    fn cooldown(&mut self) -> &mut Cooldown;
}

impl HasCooldown for Weapon {
    fn cooldown(&mut self) -> &mut Cooldown {
        &mut self.cooldown
    }
}

#[derive(Component, Debug, Clone)]
pub struct Cooldown {
    total_time: f32,
    elapsed: f32,
    ready: bool,
}

impl Cooldown {
    pub fn new(total_time: f32) -> Self {
        Self {
            total_time,
            elapsed: 0.0,
            ready: true,
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.ready = false;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_cooldowns::<Weapon>);
    }
}

fn update_cooldowns<T: Component + HasCooldown>(mut query: Query<&mut T>, time: Res<Time>) {
    for mut component in &mut query {
        let cooldown = component.cooldown();
        if !cooldown.ready {
            cooldown.elapsed += time.delta_seconds();
            if cooldown.elapsed >= cooldown.total_time {
                cooldown.ready = true;
                cooldown.elapsed = cooldown.total_time;
            }
        }
    }
}
