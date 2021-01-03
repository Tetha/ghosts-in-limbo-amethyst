use amethyst::assets::Handle;
use amethyst::ecs::{Component, DenseVecStorage};

use crate::level::{LevelDefinition};


#[derive(Debug)]
pub struct LevelAssociation {
    pub level_index: usize,
    pub level_definition: LevelDefinition,
}

impl Component for LevelAssociation {
    type Storage = DenseVecStorage<Self>;
}