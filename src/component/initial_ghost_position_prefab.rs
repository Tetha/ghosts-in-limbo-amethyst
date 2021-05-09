use amethyst::{assets::{Handle, PrefabData}, core::Transform, ecs::WriteStorage, renderer::{SpriteRender, SpriteSheet, palette::Srgb, resources::Tint}, shred::ReadExpect};
use serde::{Deserialize, Serialize};

use super::{GridPosition, ghost_color::{GhostColor, GhostColorComponent}, initial_ghost_direction::GhostDirection, initial_ghost_position::InitialGhostPosition};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialGhostPositionPrefab {
    position: GridPosition,
    color: GhostColor,
    direction: GhostDirection,
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
        ReadExpect<'a, Handle<SpriteSheet>>,
    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: amethyst::ecs::Entity,
        (grid_positions, ghost_directions, initial_ghost_positions, ghost_colors, transforms, sprite_renders, tints, sprite_sheet): &mut Self::SystemData,
        _entities: &[amethyst::ecs::Entity],
        _children: &[amethyst::ecs::Entity],
    ) -> Result<Self::Result, amethyst::Error> {
        grid_positions.insert(entity, self.position)?;
        ghost_directions.insert(entity, self.direction)?;
        initial_ghost_positions.insert(entity, InitialGhostPosition)?;
        transforms.insert(entity, Transform::default())?;
        ghost_colors.insert(entity, GhostColorComponent{ color: self.color })?;
        sprite_renders.insert(entity, SpriteRender::new(sprite_sheet.clone(), 0))?;

        match self.color {
            GhostColor::Blue => tints.insert(entity, Tint(Srgb::new(0.0, 0.0, 1.0).into()))?,
            GhostColor::Red => tints.insert(entity, Tint(Srgb::new(1.0, 0.0, 0.0).into()))?,
            GhostColor::Green => tints.insert(entity, Tint(Srgb::new(0.0, 1.0, 0.0).into()))?,
        };
        
        Ok(())
    }
}