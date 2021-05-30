use amethyst::{core::{Hidden, Parent, Transform}, ecs::{Entities, Join, ReadStorage, WriteStorage}, shred::System};
use log::warn;

use crate::component::{Toolbox, ToolboxPosition};


pub struct ToolboxElementVisibility;

impl<'a> System<'a> for ToolboxElementVisibility {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, Toolbox>,
        ReadStorage<'a, ToolboxPosition>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (
        entities,
        parents,
        toolboxes,
        toolbox_positions,
        mut hidden,
        mut transforms,
    ): Self::SystemData) {
        for (element,
            element_parent,
            toolbox_position,
            element_transform) in (&entities, &parents, &toolbox_positions, &mut transforms).join() {
            let possible_toolbox = toolboxes.get(element_parent.entity);
            if let Some(toolbox) = possible_toolbox {
                if toolbox_position.index < toolbox.offset {
                    // hide, negative overall offset
                    if !hidden.contains(element) {
                        let hide = hidden.insert(element, Hidden::default());
                        if let Err(e) = hide {
                            warn!("Toolbox visibility: failed to hide component: {}", e);
                        }
                    }
                } else {
                    let relative_index = toolbox_position.index - toolbox.offset;
                    if relative_index < toolbox.capacity {
                        // Unhide and adjust position
                        if hidden.contains(element) {
                            hidden.remove(element);
                        }
                        element_transform.set_translation_y(60.0 * relative_index as f32);
                    } else {
                        // offset too big => falls of the other end => hide as well
                        if !hidden.contains(element) {
                            let hide = hidden.insert(element, Hidden::default());
                            if let Err(e) = hide {
                                warn!("Toolbox visibility: failed to hide component: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}