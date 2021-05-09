use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::derive::PrefabData;
use amethyst::ecs::{Component, DenseVecStorage, Entity};
use amethyst::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PrefabData)]
pub enum GhostColor {
    Blue,
    Red,
    Green,
}

#[derive(Debug)]
pub struct GhostColorComponent {
    pub color: GhostColor
}

impl Component for GhostColorComponent {
    type Storage = DenseVecStorage<Self>;
}