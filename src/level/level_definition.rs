use amethyst::assets::{Asset, Handle};
use amethyst::ecs::VecStorage;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDefinition {
    pub levels: Vec<LevelDefinition>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelDefinition {
    pub name: String
}

pub type WorldDefinitionHandle = Handle<WorldDefinition>;

impl Asset for WorldDefinition {
    const NAME: &'static str = "ghosts::WorldDefinition";

    type Data = Self;

    type HandleStorage = VecStorage<WorldDefinitionHandle>;
}