use amethyst::assets::{Asset, Handle};
use amethyst::ecs::VecStorage;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDefinition {
    pub levels: Vec<LevelMetadata>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelMetadata {
    pub name: String,
    pub id: String
}

pub type WorldDefinitionHandle = Handle<WorldDefinition>;

impl Asset for WorldDefinition {
    const NAME: &'static str = "ghosts::WorldDefinition";

    type Data = Self;

    type HandleStorage = VecStorage<WorldDefinitionHandle>;
}