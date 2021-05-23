use amethyst::{assets::{Handle, PrefabData}, core::Transform, ecs::WriteStorage, renderer::{SpriteRender, SpriteSheet}, shred::{ReadExpect}};
use amethyst::ecs::Entity;
use amethyst::Error;
use serde::{Deserialize, Serialize};

use crate::game::{ArrowDirection, MemoryType, TJunctionDirection, TJunctionExit};

use super::{GoalTile, GridPosition, MemoryTile, SimpleArrowTile};

// TODO: follow the tutorial at [1]
// TODO: add this to the level definition
// TODO: instantiate the tiles to create the loaded level
#[derive(Debug, Deserialize, Serialize)]
pub enum StaticGridTilePrefab {
    TurnArrow {
        grid_position: GridPosition,
        arrow_direction: ArrowDirection,
    },
    Memory {
        grid_position: GridPosition,
        memory: MemoryType,
    },
    Junction {
        grid_position: GridPosition,
        required_memory: MemoryType,
        direction: TJunctionDirection,
        exit: TJunctionExit,
        memory_on_turn: bool,
    },
    Exit {
        grid_position: GridPosition,
    }
}

impl<'a> PrefabData<'a> for StaticGridTilePrefab {
    type SystemData = (
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, GoalTile>,
        WriteStorage<'a, MemoryTile>,
        WriteStorage<'a, SimpleArrowTile>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Handle<SpriteSheet>>
    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (grid_positions,
            goal_tiles,
            memory_tiles,
            simple_arrow_tiles,
            sprite_renderes,
            transforms,
            sprite_sheet): &mut Self::SystemData,

        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        println!("Add to entity for arrow is being called");
        match self {
            StaticGridTilePrefab::TurnArrow { grid_position, arrow_direction } => {
                grid_positions.insert(entity, grid_position.clone())?;
                simple_arrow_tiles.insert(entity, SimpleArrowTile{ direction: arrow_direction.clone() })?;
                sprite_renderes.insert(entity, SpriteRender::new(sprite_sheet.clone(), arrow_direction.clone().into()))?;
            }
            StaticGridTilePrefab::Memory { grid_position, memory } => {
                grid_positions.insert(entity, grid_position.clone())?;
                memory_tiles.insert(entity, MemoryTile{ memory_type: memory.clone() })?;
                sprite_renderes.insert(entity, 
                    SpriteRender::new(sprite_sheet.clone(), 15))?;
            }
            StaticGridTilePrefab::Junction { grid_position, required_memory, direction, exit, memory_on_turn } => { todo!() }
            StaticGridTilePrefab::Exit { grid_position } => {
                grid_positions.insert(entity, grid_position.clone())?;
                sprite_renderes.insert(entity,
                    SpriteRender::new(sprite_sheet.clone(), 14))?;
                goal_tiles.insert(entity, GoalTile{})?;
            }
        }
        transforms.insert(entity, Transform::default())?;
        Ok(())
    }
}