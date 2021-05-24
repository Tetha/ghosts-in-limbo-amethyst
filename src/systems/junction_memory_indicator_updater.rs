use amethyst::{core::{Parent, Transform, math::Vector3}, ecs::{Entities, Join, ReadStorage, System, WriteStorage}, renderer::{SpriteRender, palette::{Srgba}, resources::Tint}};

use crate::{component::{GhostColor, GhostColorComponent, JunctionMemoryIndicator, JunctionTile, MemoryTile, MemoryTypeIndicator, SpriteSelector}, game::TJunctionMemoryPlacement};


#[derive(Copy, Clone, Debug, Default)]
pub struct JunctionMemoryUpdater;

impl<'a> System<'a> for JunctionMemoryUpdater {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, JunctionMemoryIndicator>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, JunctionTile>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Tint>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (
        entities,
        junction_memory_indicators,
        parents,
        junction_tiles,
        mut sprite_renders,
        mut tints,
        mut transforms,
    ): Self::SystemData) {
        for (indicator, _, parent) in (&entities, &junction_memory_indicators, &parents).join() {
            let parent_entity = parent.entity;
            let parent_tile = junction_tiles.get(parent_entity);

            let indicator_render = sprite_renders.get_mut(indicator);
            let indicator_transform = transforms.get_mut(indicator);
            let indicator_tint = tints.get_mut(indicator);

            match (parent_tile, indicator_render, indicator_transform, indicator_tint) {
                (Some(parent_tile), 
                 Some(indicator_render),
                 Some(indicator_transform),
                 Some(indicator_tint)) => {
                    indicator_render.sprite_number = parent_tile.memory.sprite_index();

                    indicator_transform.set_scale(Vector3::new(0.25, 0.25, 1.));
                    match parent_tile.memory_position {
                        TJunctionMemoryPlacement::Straight => indicator_transform.set_translation_xyz(11., 0., 0.),
                        TJunctionMemoryPlacement::Turn => indicator_transform.set_translation_xyz(15., -20., 0.),
                    };

                    indicator_tint.0 = match parent_tile.ghost {
                        GhostColor::Green => Srgba::new(0.0, 1.0, 0.0, 1.0),
                        GhostColor::Blue => Srgba::new(0.0, 0.0, 1.0, 1.0),
                        GhostColor::Red => Srgba::new(1.0, 0.0, 0.0, 1.0),
                    }
                },
                _ => {}
            }
        }
    }
}