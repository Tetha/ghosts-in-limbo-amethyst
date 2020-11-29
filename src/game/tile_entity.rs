use amethyst::assets::Handle;
use amethyst::core::{Parent, Transform};
use amethyst::prelude::*;
use amethyst::ecs::Entity;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::shred::World;

pub enum ArrowDirection {
	ArrowDirectionStraightUp,
	ArrowDirectionStraightDown,
	ArrowDirectionStraightRight,
	ArrowDirectionStraightLeft,

	ArrowDirectionLeftTurnFromBottom,
	ArrowDirectionLeftTurnFromTop,
	ArrowDirectionLeftTurnFromLeft,
	ArrowDirectionLeftTurnFromRight,

	ArrowDirectionRightTurnFromBottom,
	ArrowDirectionRightTurnFromTop,
	ArrowDirectionRightTurnFromLeft,
	ArrowDirectionRightTurnFromRight,
}

pub fn create_arrow_entity(
    world: &mut World,
    sprite_sheet: Handle<SpriteSheet>,
    direction: ArrowDirection,
    transform: Transform,
    parent: Option<Entity>) -> Entity {

    let mut builder = world.create_entity()
                                    .with(transform)
                                    .with(SpriteRender::new(sprite_sheet, direction.into()));
    if let Some(parent) = parent {
        builder = builder.with(Parent::new(parent));
    }
    builder.build()
}

impl Into<usize> for ArrowDirection {
    fn into(self) -> usize {
        match self {
            ArrowDirection::ArrowDirectionStraightUp => 1,
            ArrowDirection::ArrowDirectionStraightRight => 2,
            ArrowDirection::ArrowDirectionStraightDown => 3,
            ArrowDirection::ArrowDirectionStraightLeft => 4,
            ArrowDirection::ArrowDirectionLeftTurnFromBottom => 5,
            ArrowDirection::ArrowDirectionLeftTurnFromLeft => 6,
            ArrowDirection::ArrowDirectionLeftTurnFromTop => 7,
            ArrowDirection::ArrowDirectionLeftTurnFromRight => 8,
            ArrowDirection::ArrowDirectionRightTurnFromBottom => 9,
            ArrowDirection::ArrowDirectionRightTurnFromLeft => 10,
            ArrowDirection::ArrowDirectionRightTurnFromTop => 11,
            ArrowDirection::ArrowDirectionRightTurnFromRight => 12,
        }
    }
}