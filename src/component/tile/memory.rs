use amethyst::ecs::{Component, DenseVecStorage, NullStorage};

use crate::game::MemoryType;

#[derive(Debug, Clone, Copy)]
pub struct MemoryTile {
    pub memory_type: MemoryType,
}

impl Component for MemoryTile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct MemoryTypeIndicator {}

impl Component for MemoryTypeIndicator {
    type Storage = NullStorage<Self>;
}