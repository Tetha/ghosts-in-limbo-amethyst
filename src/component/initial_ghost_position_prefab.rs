use amethyst::{assets::{Handle, PrefabData}, core::Transform, ecs::WriteStorage, renderer::{SpriteRender, SpriteSheet}, shred::ReadExpect};
use serde::{Deserialize, Serialize};

use super::{GridPosition, ghost_color::{GhostColor, GhostColorComponent}, initial_ghost_position::InitialGhostPosition};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialGhostPositionPrefab {
    position: GridPosition,
    color: GhostColor
}

impl<'a> PrefabData<'a> for InitialGhostPositionPrefab {
    type SystemData = (
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, InitialGhostPosition>,
        WriteStorage<'a, GhostColorComponent>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        ReadExpect<'a, Handle<SpriteSheet>>,
    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: amethyst::ecs::Entity,
        (grid_positions, initial_ghost_positions, ghost_colors, transforms, sprite_renders, sprite_sheet): &mut Self::SystemData,
        _entities: &[amethyst::ecs::Entity],
        _children: &[amethyst::ecs::Entity],
    ) -> Result<Self::Result, amethyst::Error> {
        grid_positions.insert(entity, self.position)?;
        initial_ghost_positions.insert(entity, InitialGhostPosition)?;
        transforms.insert(entity, Transform::default())?;
        ghost_colors.insert(entity, GhostColorComponent{ color: self.color })?;
        sprite_renders.insert(entity, SpriteRender::new(sprite_sheet.clone(), 0))?;
        Ok(())
    }
}