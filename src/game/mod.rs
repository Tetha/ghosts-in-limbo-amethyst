mod tile_entity;
mod t_junction;

pub use tile_entity::{ArrowDirection, create_arrow_entity};
pub use t_junction::{TJunctionExit, TJunctionDirection, TJunctionMemoryPlacement, create_junction_entity};

use crate::{component::SpriteSelector, level::{WorldDefinitionHandle}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MemoryType {
    Love,
    Pet,
    Sun,
}

impl SpriteSelector for MemoryType {
    fn sprite_index(&self) -> usize {
        match self {
            crate::game::MemoryType::Love => 16, 
            crate::game::MemoryType::Pet => 17,
            crate::game::MemoryType::Sun => 18,
        }

    }
}
#[derive(Debug, Default)]
pub struct MainMenuData {
    pub current_world: Option<WorldDefinitionHandle>
}