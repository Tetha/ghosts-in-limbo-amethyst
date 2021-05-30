use amethyst::{assets::{Handle, PrefabData}, core::{Transform}, ecs::{WriteStorage}, renderer::{SpriteRender, SpriteSheet, palette::{Srgba}, resources::Tint}, shred::ReadExpect};
use serde::{Deserialize, Serialize};

use super::{GhostDirectionIndicator, GridPosition, ghost_color::{GhostColor, GhostColorComponent}, initial_ghost_direction::GhostDirection, initial_ghost_position::InitialGhostPosition};

#[derive(Debug, Serialize, Deserialize)]
pub enum InitialGhostPositionPrefab {
    GhostPosition{
        position: GridPosition,
        color: GhostColor,
        direction: GhostDirection,
    },
    DirectionIndicator{
    },
}

impl<'a> PrefabData<'a> for InitialGhostPositionPrefab {
    type SystemData = (
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, GhostDirection>,
        WriteStorage<'a, InitialGhostPosition>,
        WriteStorage<'a, GhostColorComponent>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Tint>,
        WriteStorage<'a, GhostDirectionIndicator>,
        ReadExpect<'a, Handle<SpriteSheet>>,
    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: amethyst::ecs::Entity,
        (grid_positions,
         ghost_directions,
         initial_ghost_positions,
         ghost_colors,
         transforms,
         sprite_renders,
         tints,
         ghost_direction_indicators,
         sprite_sheet): &mut Self::SystemData,
        _entities: &[amethyst::ecs::Entity],
        _children: &[amethyst::ecs::Entity],
    ) -> Result<Self::Result, amethyst::Error> {
        match self {
            InitialGhostPositionPrefab::GhostPosition { position, color, direction } => {
                grid_positions.insert(entity, position.clone())?;
                ghost_directions.insert(entity, direction.clone())?;
                initial_ghost_positions.insert(entity, InitialGhostPosition)?;
                transforms.insert(entity, Transform::default())?;
                ghost_colors.insert(entity, GhostColorComponent{ color: color.clone() })?;
                sprite_renders.insert(entity, SpriteRender::new(sprite_sheet.clone(), 0))?;

                tints.insert(entity, Tint(color.to_color()))?;
            }
            InitialGhostPositionPrefab::DirectionIndicator {  } => {
                ghost_direction_indicators.insert(entity, GhostDirectionIndicator)?;
                transforms.insert(entity, Transform::default())?;
                sprite_renders.insert(entity, SpriteRender::new(sprite_sheet.clone(), 0))?;
                tints.insert(entity, Tint(Srgba::new(0.0, 0.0, 0.0, 0.0)))?;
            }
        }
        Ok(())
    }
}