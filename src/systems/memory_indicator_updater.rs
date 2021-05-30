use amethyst::{core::{Parent, Transform, math::Vector3}, ecs::{Entities, Join, ReadStorage, System, WriteStorage}, renderer::{SpriteRender, resources::Tint}};

use crate::{component::{GhostColorComponent, MemoryTile, MemoryTypeIndicator, SpriteSelector}};


#[derive(Copy, Clone, Debug, Default)]
pub struct MemoryIndicatorUpdater;

impl<'a> System<'a> for MemoryIndicatorUpdater {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, GhostColorComponent>,
        ReadStorage<'a, MemoryTypeIndicator>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, MemoryTile>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Tint>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (
        entities,
        ghost_colors,
        memory_type_indicators,
        parents,
        memory_tiles,
        mut sprite_renders,
        mut tints,
        mut transforms,
    ): Self::SystemData) {
        for (indicator, _, parent) in (&entities, &memory_type_indicators, &parents).join() {
            let parent_entity = parent.entity;
            let parent_tile = memory_tiles.get(parent_entity);
            let parent_color = ghost_colors.get(parent_entity);

            let indicator_render = sprite_renders.get_mut(indicator);
            let indicator_tint = tints.get_mut(indicator);
            let indicator_transform = transforms.get_mut(indicator);

            match (parent_tile, parent_color, indicator_render, indicator_tint, indicator_transform) {
                (Some(parent_tile),
                 Some(parent_color),
                 Some(indicator_render),
                 Some(indicator_tint),
                 Some(indicator_transform)) => {
                    indicator_render.sprite_number = parent_tile.memory_type.sprite_index();

                    indicator_tint.0 = parent_color.color.to_color();
                    indicator_transform.set_translation_xyz(0.0, 5.0, 0.0);
                    indicator_transform.set_scale(Vector3::new(0.2, 0.2, 1.0));
                }
                _ => {}
            }
        }
    }
}