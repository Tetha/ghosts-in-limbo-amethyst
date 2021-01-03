use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::derive::PrefabData;
use amethyst::ecs::{Component, DenseVecStorage, Entity};
use amethyst::Error;

use crate::game::{TJunctionDirection, TJunctionExit, TJunctionMemoryPlacement};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PrefabData, Deserialize, Serialize)]
pub struct JunctionTile {
    exit: TJunctionExit,
    turn: TJunctionDirection,
    memory_position: TJunctionMemoryPlacement,
}

impl Component for JunctionTile {
    type Storage = DenseVecStorage<Self>;
}