use amethyst::{core::{Parent, Transform, math::Vector3}, ecs::{Entities, Join, ReadStorage, System, WriteStorage}, renderer::{SpriteRender, palette::{Srgb, Srgba}, resources::Tint}};

use crate::component::{GhostColor, GhostColorComponent, GhostDirection, GhostDirectionIndicator};


#[derive(Copy, Clone, Debug, Default)]
pub struct GhostDirectionIndicatorUpdater;

impl<'a> System<'a> for GhostDirectionIndicatorUpdater {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, GhostDirectionIndicator>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, GhostDirection>,
        ReadStorage<'a, GhostColorComponent>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Tint>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (
        entities,
        ghost_direction_indicators,
        parents,
        ghost_directions,
        ghost_colors,
        mut sprite_renders,
        mut tints,
        mut transforms,
    ): Self::SystemData) {
        for (indicator, _, parent) in (&entities, &ghost_direction_indicators, &parents).join() {
            let parent_entity = parent.entity;
            let parent_direction = ghost_directions.get(parent_entity);
            let parent_color = ghost_colors.get(parent_entity);

            let indicator_render = sprite_renders.get_mut(indicator);
            let indicator_tint = tints.get_mut(indicator);
            let indicator_transform = transforms.get_mut(indicator);

            match (parent_direction, parent_color, indicator_render, indicator_tint, indicator_transform) {
                (Some(parent_direction),
                 Some(parent_color),
                 Some(indicator_render),
                 Some(indicator_tint),
                 Some(indicator_transform)) => {
                    indicator_render.sprite_number = match parent_direction {
                        GhostDirection::Up => 1,
                        GhostDirection::Down => 2,
                        GhostDirection::Left => 3,
                        GhostDirection::Right => 4,
                    };

                    indicator_tint.0 = match parent_color.color {
                        GhostColor::Green => Srgba::new(0.0, 1.0, 0.0, 1.0),
                        GhostColor::Blue => Srgba::new(0.0, 0.0, 1.0, 1.0),
                        GhostColor::Red => Srgba::new(1.0, 0.0, 0.0, 1.0),
                    };

                    indicator_transform.set_translation_xyz(0.0, -30.0, 0.0);
                    indicator_transform.set_scale(Vector3::new(0.6, 0.6, 1.0));
                 }

                 _ => {}
            }
        }
    }
}