use std::f32::consts::PI;

use amethyst::{assets::{Handle, PrefabData}, core::{Transform, math::Vector3}, ecs::WriteStorage, renderer::{SpriteRender, SpriteSheet, palette::Srgb, resources::Tint}, shred::{ReadExpect}};
use amethyst::ecs::Entity;
use amethyst::Error;
use serde::{Deserialize, Serialize};

use crate::{component::JunctionMemoryIndicator, game::{ArrowDirection, MemoryType, TJunctionDirection, TJunctionExit, TJunctionMemoryPlacement}};

use super::{GhostColor, GhostColorComponent, GoalTile, GridPosition, JunctionTile, MemoryTile, MemoryTypeIndicator, SimpleArrowTile, toolbox_position::ToolboxPosition};

#[derive(Debug, Serialize, Deserialize)]
pub enum TilePosition {
    Grid { x: usize, y: usize },
    Toolbox { toolbox: usize, index: i32 },
}
// TODO: follow the tutorial at [1]
// TODO: add this to the level definition
// TODO: instantiate the tiles to create the loaded level
#[derive(Debug, Deserialize, Serialize)]
pub enum StaticGridTilePrefab {
    TurnArrow {
        position: TilePosition,
        //grid_position: GridPosition,
        arrow_direction: ArrowDirection,
    },
    Memory {
        position: TilePosition,
        //grid_position: GridPosition,
        memory: MemoryType,
        ghost: GhostColor,
    },
    MemoryIndicator {},
    Junction {
        position: TilePosition,
        //grid_position: GridPosition,
        required_memory: MemoryType,
        direction: TJunctionDirection,
        exit: TJunctionExit,
        memory_on_turn: bool,
        ghost: GhostColor,
    },
    JunctionIndicator {},
    Exit {
        position: TilePosition,
        //grid_position: GridPosition,
    }
}

impl<'a> PrefabData<'a> for StaticGridTilePrefab {
    type SystemData = (
        WriteStorage<'a, GhostColorComponent>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, GoalTile>,
        WriteStorage<'a, JunctionTile>,
        WriteStorage<'a, JunctionMemoryIndicator>,
        WriteStorage<'a, MemoryTile>,
        WriteStorage<'a, MemoryTypeIndicator>,
        WriteStorage<'a, SimpleArrowTile>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, ToolboxPosition>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tint>,
        ReadExpect<'a, Handle<SpriteSheet>>
    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (
            ghost_colors,
            grid_positions,
            goal_tiles,
            junction_tiles,
            junction_memory_indicators,
            memory_tiles,
            memory_type_indicators,
            simple_arrow_tiles,
            sprite_renderes,
            toolbox_positions,
            transforms,
            tints,
            sprite_sheet): &mut Self::SystemData,

        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        println!("Add to entity for arrow is being called");
        match self {
            StaticGridTilePrefab::TurnArrow { position, arrow_direction } => {
                transforms.insert(entity, Transform::default())?;
                load_position(entity, grid_positions, toolbox_positions, position)?;
                simple_arrow_tiles.insert(entity, SimpleArrowTile{ direction: arrow_direction.clone() })?;
                sprite_renderes.insert(entity, SpriteRender::new(sprite_sheet.clone(), arrow_direction.clone().into()))?;
            }
            StaticGridTilePrefab::Memory { position, memory, ghost: color } => {
                transforms.insert(entity, Transform::default())?;
                load_position(entity, grid_positions, toolbox_positions, position)?;
                memory_tiles.insert(entity, MemoryTile{ memory_type: *memory })?;
                ghost_colors.insert(entity, GhostColorComponent{ color: *color })?;
                sprite_renderes.insert(entity, 
                    SpriteRender::new(sprite_sheet.clone(), 15))?;
            },
            StaticGridTilePrefab::MemoryIndicator {} => {
                transforms.insert(entity, Transform::default())?;
                memory_type_indicators.insert(entity, MemoryTypeIndicator{})?;
                tints.insert(entity, Tint(Srgb::new(1.0, 1.0, 1.0).into()))?;
                sprite_renderes.insert(entity,
                    SpriteRender::new(sprite_sheet.clone(), 17))?;
            },
            StaticGridTilePrefab::Junction {
                position,
                required_memory,
                direction,
                exit,
                memory_on_turn,
                ghost,
            } => {
                load_junction(
                    entity, position, required_memory, direction, exit, memory_on_turn, ghost,
                    &sprite_sheet, junction_tiles, grid_positions, sprite_renderes, toolbox_positions, transforms)?;
            },
            StaticGridTilePrefab::JunctionIndicator {} => {
                transforms.insert(entity, Transform::default())?;
                junction_memory_indicators.insert(entity, JunctionMemoryIndicator{})?;
                tints.insert(entity, Tint(Srgb::new(1.0, 1.0, 1.0).into()))?;
                sprite_renderes.insert(entity,
                    SpriteRender::new(sprite_sheet.clone(), 17))?;
            }
            StaticGridTilePrefab::Exit { position } => {
                transforms.insert(entity, Transform::default())?;
                goal_tiles.insert(entity, GoalTile{})?;
                load_position(entity, grid_positions, toolbox_positions, position)?;
                sprite_renderes.insert(entity,
                    SpriteRender::new(sprite_sheet.clone(), 14))?;
            }
        }
        Ok(())
    }
}

fn load_position<'a>(
    entity: Entity,
    grid_positions: &mut WriteStorage<'a, GridPosition>,
    toolbox_positions: &mut WriteStorage<'a, ToolboxPosition>,
    position: &TilePosition) -> Result<(), Error> {
    match position {
        TilePosition::Grid { x, y } => {
            grid_positions.insert(entity, GridPosition{x: *x, y: *y})?;
        }
        TilePosition::Toolbox { toolbox, index } => {
            toolbox_positions.insert(entity, ToolboxPosition{ toolbox: *toolbox, index: *index })?;
        }
    }
    Ok(())
}
fn load_junction<'a>(
    entity: Entity,
    position: &TilePosition,
    required_memory: &MemoryType,
    direction: &TJunctionDirection,
    exit: &TJunctionExit,
    memory_on_turn: &bool,
    ghost: &GhostColor,
    sprite_sheet: &Handle<SpriteSheet>,
    junction_tiles: &mut WriteStorage<'a, JunctionTile>,
    grid_positions: &mut WriteStorage<'a, GridPosition>,
    sprite_renderes: &mut WriteStorage<'a, SpriteRender>,
    toolbox_positions: &mut WriteStorage<'a, ToolboxPosition>,
    transforms: &mut WriteStorage<'a, Transform>,
) -> Result<(), Error> {
    println!("T Junction not done yet");

    load_position(entity, grid_positions, toolbox_positions, position)?;
    sprite_renderes.insert(entity, SpriteRender::new(sprite_sheet.clone(), 13))?;


    let mut transform = Transform::default();

    match direction {
        TJunctionDirection::LeftTurn => transform.set_scale(Vector3::new(-1., 1., 1.)),
        TJunctionDirection::RightTurn => (),  // nothing to do
    }
    
    match exit {
        TJunctionExit::Up => (), // nothing
        TJunctionExit::Down => { transform.set_rotation_2d(PI); },
        TJunctionExit::Left => { transform.set_rotation_2d(0.5 * PI); }
        TJunctionExit::Right => { transform.set_rotation_2d(1.5 * PI); }
    }
    transforms.insert(entity, transform)?;

    junction_tiles.insert(entity, JunctionTile{
        exit: *exit,
        turn: *direction,
        memory_position: match memory_on_turn {
            true => TJunctionMemoryPlacement::Turn,
            false => TJunctionMemoryPlacement::Straight,
        },
        memory: *required_memory,
        ghost: *ghost,
    })?;
    Ok(())
}