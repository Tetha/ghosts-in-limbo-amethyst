use amethyst::ecs::{Component, DenseVecStorage};

use crate::game::MemoryType;

#[derive(Debug, Clone, Copy)]
pub struct MemoryTile {
    pub memory_type: MemoryType,
}

impl Component for MemoryTile {
    type Storage = DenseVecStorage<Self>;
}