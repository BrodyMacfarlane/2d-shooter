pub enum ActionName {
    Backflip,
    Dash,
    Shoot,
    Move
}

pub struct Action {
    name: ActionName,
    direction: Vec2,
    priority: u32
}
