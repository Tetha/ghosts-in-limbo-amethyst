use amethyst::assets::{Asset, Handle};
use amethyst::ecs::VecStorage;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldDefinition {
    levels: Vec<LevelDefinition>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelDefinition {
    name: String
}

pub type WorldDefinitionHandle = Handle<WorldDefinition>;

impl Asset for WorldDefinition {
    const NAME: &'static str = "ghosts::WorldDefinition";

    type Data = Self;

    type HandleStorage = VecStorage<WorldDefinitionHandle>;
}