use amethyst::ecs::{Component, DenseVecStorage};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GhostDirection {
    Up, Down, Left, Right
}

impl Component for GhostDirection {
    type Storage = DenseVecStorage<Self>;
}