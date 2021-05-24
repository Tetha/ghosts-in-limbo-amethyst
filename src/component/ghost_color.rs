use amethyst::{assets::{PrefabData, ProgressCounter}, renderer::palette::Srgba};
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

impl GhostColor {
    pub fn to_color(&self) -> Srgba {
        match self {
            GhostColor::Green => Srgba::new(0.0, 1.0, 0.0, 1.0),
            GhostColor::Blue => Srgba::new(0.0, 0.0, 1.0, 1.0),
            GhostColor::Red => Srgba::new(1.0, 0.0, 0.0, 1.0),
        }
    }
}