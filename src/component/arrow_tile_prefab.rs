use amethyst::{assets::{Handle, PrefabData}, core::Transform, ecs::WriteStorage, renderer::{SpriteRender, SpriteSheet}, shred::{ReadExpect}};
use amethyst::ecs::Entity;
use amethyst::Error;
use serde::{Deserialize, Serialize};

use crate::game::ArrowDirection;

use super::{GridPosition, SimpleArrowTile};

// TODO: follow the tutorial at [1]
// TODO: add this to the level definition
// TODO: instantiate the tiles to create the loaded level
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrowTilePrefab {
    grid_position: GridPosition,
    arrow_direction: ArrowDirection,
}

impl<'a> PrefabData<'a> for ArrowTilePrefab {
    type SystemData = (
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, SimpleArrowTile>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Handle<SpriteSheet>>
    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (grid_positions, simple_arrow_tiles, sprite_renderes, transforms, sprite_sheet): &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        println!("Add to entity for arrow is being called");
        grid_positions.insert(entity, self.grid_position)?;
        simple_arrow_tiles.insert(entity, SimpleArrowTile{ direction: self.arrow_direction })?;
        sprite_renderes.insert(entity, SpriteRender::new(sprite_sheet.clone(), self.arrow_direction.into()))?;
        
        transforms.insert(entity, Transform::default())?;
        Ok(())
    }
}