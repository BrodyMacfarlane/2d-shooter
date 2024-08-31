pub enum StatName {
    HP,
    Damage,
    Armor,
    Speed,
    FireRate
}

#[derive(Component)]
pub struct Stat {
    name: StatName,
    value: f32,
    modifiers: Vec<f32>,
}
