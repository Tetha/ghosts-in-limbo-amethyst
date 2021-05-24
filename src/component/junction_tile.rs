use amethyst::{ecs::NullStorage};
use amethyst::ecs::{Component, DenseVecStorage};

use crate::game::{MemoryType, TJunctionDirection, TJunctionExit, TJunctionMemoryPlacement};

use serde::{Deserialize, Serialize};

use super::GhostColor;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct JunctionTile {
    pub exit: TJunctionExit,
    pub turn: TJunctionDirection,
    pub memory_position: TJunctionMemoryPlacement,
    pub memory: MemoryType,
    pub ghost: GhostColor,
}

impl Component for JunctionTile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct JunctionMemoryIndicator {}

impl Component for JunctionMemoryIndicator {
    type Storage = NullStorage<Self>;
}