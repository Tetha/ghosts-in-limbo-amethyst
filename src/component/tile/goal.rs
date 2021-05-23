use amethyst::ecs::{Component, NullStorage};


#[derive(Debug, Default)]
pub struct GoalTile {
}

impl Component for GoalTile {
    type Storage = NullStorage<Self>;
}