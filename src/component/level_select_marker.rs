use amethyst::assets::PrefabData;
use amethyst::derive::PrefabData;
use amethyst::ecs::{Component, Entity, NullStorage, WriteStorage};
use amethyst::Error;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct LevelSelectButtonMarker;

impl Component for LevelSelectButtonMarker {
    type Storage = NullStorage<Self>;
}