pub enum ActionName {
    Backflip,
    Dash,
    Shoot,
    Move
}

pub struct Action {
    name: ActionName,
    priority: u32
}
