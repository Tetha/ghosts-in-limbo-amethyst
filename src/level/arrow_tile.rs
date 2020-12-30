use amethyst::ecs::{Component, DenseVecStorage, Entity, WriteStorage};
use crate::game::ArrowDirection;
use serde::{Deserialize, Serialize};
use amethyst::Error;
use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::derive::PrefabData;
pub struct SimpleArrowTile {
    pub direction: ArrowDirection,
}

impl Component for SimpleArrowTile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ArrowTilePrefab {
	StraightUp,
	StraightDown,
	StraightRight,
	StraightLeft,

	LeftTurnFromBottom,
	LeftTurnFromTop,
	LeftTurnFromLeft,
	LeftTurnFromRight,

	RightTurnFromBottom,
	RightTurnFromTop,
	RightTurnFromLeft,
	RightTurnFromRight,
}

impl<'a> PrefabData<'a> for ArrowTilePrefab {
    type SystemData = WriteStorage<'a, SimpleArrowTile>;

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        arrow_tiles: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        let direction = match *self {
            ArrowTilePrefab::StraightUp => ArrowDirection::ArrowDirectionStraightUp,
            ArrowTilePrefab::StraightDown => ArrowDirection::ArrowDirectionStraightDown,
            ArrowTilePrefab::StraightRight => ArrowDirection::ArrowDirectionStraightRight,
            ArrowTilePrefab::StraightLeft => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::LeftTurnFromBottom => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::LeftTurnFromTop => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::LeftTurnFromLeft => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::LeftTurnFromRight => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::RightTurnFromBottom => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::RightTurnFromTop => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::RightTurnFromLeft => ArrowDirection::ArrowDirectionStraightLeft,
            ArrowTilePrefab::RightTurnFromRight => ArrowDirection::ArrowDirectionStraightLeft,
        };
        arrow_tiles.insert(entity, SimpleArrowTile{ direction }).map(|_| ())?;
        Ok(())
    }
}