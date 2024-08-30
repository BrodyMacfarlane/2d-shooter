use bevy::prelude::*;
use super::components::*;

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Animator, &mut Sprite)>,
) {
    for (mut animator, mut sprite) in query.iter_mut() {
        if let Some(state) = animator.states.iter_mut().find(|s| s.current == animator.current_state) {
            state.timer.tick(time.delta());
            if state.timer.just_finished() {
                sprite.index = calculate_sprite_index(&animator.current_state, sprite.index);
            }
        }
    }
}

fn calculate_sprite_index(animation_type: &AnimationType, current_index: usize) -> usize {
    let (start_index, frame_count) = match animation_type {
        AnimationType::Idle => (0, 4),
        AnimationType::Walking => (4, 4),
        AnimationType::Running => (8, 4),
        AnimationType::Attacking => (12, 4),
    };
    start_index + ((current_index + 1 - start_index) % frame_count)
}
