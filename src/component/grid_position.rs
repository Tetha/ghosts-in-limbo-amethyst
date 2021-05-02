use amethyst::assets::PrefabData;
use amethyst::derive::PrefabData;
use amethyst::ecs::{Component, DenseVecStorage, Entity, WriteStorage};
use amethyst::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl Component for GridPosition {
    type Storage = DenseVecStorage<Self>;
}