use amethyst::ecs::{Component, DenseVecStorage, Entity};
use crate::game::ArrowDirection;
use serde::{Deserialize, Serialize};
use amethyst::Error;
use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::derive::PrefabData;
#[derive(Debug, Deserialize, Serialize, PrefabData, Clone, Copy)]
pub struct SimpleArrowTile {
    pub direction: ArrowDirection,
}

impl Component for SimpleArrowTile {
    type Storage = DenseVecStorage<Self>;
}
