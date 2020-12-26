use std::f32::consts::PI;

use amethyst::assets::Handle;
use amethyst::core::math::Vector3;
use amethyst::core::{Parent, Transform};
use amethyst::ecs::{Entity};
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::{SpriteRender, SpriteSheet};

use super::MemoryType;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TJunctionExit {
    Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TJunctionDirection {
    LeftTurn,
    RightTurn,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TJunctionMemoryPlacement {
    Straight,
    Turn,
}

pub fn create_junction_entity(
    world: &mut World,
    sprite_sheet: Handle<SpriteSheet>,
    exit: TJunctionExit,
    direction: TJunctionDirection,
    memory_placement: TJunctionMemoryPlacement,
    memory: MemoryType,
    transform: Transform,
    parent: Option<Entity>) -> Entity {
    
    let mut builder = world.create_entity()
                                        .with(transform);
    
    if let Some(parent) = parent {
        builder = builder.with(Parent::new(parent));
    }

    let result = builder.build();
    let base_sprite = add_base_sprite(world, result, sprite_sheet.clone(), exit, direction);
    add_emotion_sprite(world, base_sprite, sprite_sheet, memory, memory_placement);
    result
}

fn add_base_sprite(world: &mut World,
                   base_entity: Entity,
                   sprite_sheet: Handle<SpriteSheet>,
                   exit: TJunctionExit,
                   direction: TJunctionDirection) -> Entity {
    let sprite_index = 13;

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
    // base arrow sprite
    world.create_entity()
         .with(SpriteRender::new(sprite_sheet, sprite_index))
         .with(Parent::new(base_entity))
         .with(transform)
         .build()

}

// TODO: ghost coloring
fn add_emotion_sprite(
    world: &mut World,
    base_entity: Entity,
    sprite_sheet: Handle<SpriteSheet>,
    memory_type: MemoryType,
    memory_placement: TJunctionMemoryPlacement,
) {
    let sprite_index = match memory_type {
        MemoryType::Love => 16,
        MemoryType::Pet => 17,
        MemoryType::Sun => 18,
    };

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.25, 0.25, 1.));
    match memory_placement {
        TJunctionMemoryPlacement::Straight => transform.append_translation_xyz(11., 0., 0.),
        TJunctionMemoryPlacement::Turn => transform.append_translation_xyz(15., -15., 0.),
    };

    world.create_entity()
         .with(Parent::new(base_entity))
         .with(transform)
         .with(SpriteRender::new(sprite_sheet, sprite_index))
         .with(Tint(Srgba::new(0., 0., 1., 1.)))
         .build();
}