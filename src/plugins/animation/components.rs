use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Clone)]
pub struct AnimationState {
    pub current: AnimationType,
    pub frame: usize,
    pub timer: Timer,
}

#[derive(Component, Debug)]
pub struct Animator {
    pub states: Vec<AnimationState>,
    pub current_state: AnimationType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationType {
    Idle,
    Walking,
    Running,
    Attacking,
    // Add more types as needed
}
