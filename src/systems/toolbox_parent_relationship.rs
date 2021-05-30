use amethyst::{core::Parent, ecs::{Entities, Join, ReadStorage, WriteStorage}, shred::System};
use log::warn;

use crate::component::{Toolbox, ToolboxPosition};


pub struct ToolboxParentRelationship;

impl<'a> System<'a> for ToolboxParentRelationship {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Toolbox>,
        ReadStorage<'a, ToolboxPosition>,
        WriteStorage<'a, Parent>,
    );

    fn run(&mut self, (
        entities,
        toolboxes,
        toolbox_positions,
        mut parents,
    ): Self::SystemData) {
        for (toolbox_entity, toolbox) in (&entities, &toolboxes).join() {
            for (entity, element_position) in (&entities, &toolbox_positions).join() {
                if toolbox.id == element_position.toolbox {
                    match parents.get_mut(entity) {
                        Some(p) => {
                            if p.entity != toolbox_entity {
                                p.entity = toolbox_entity
                            }
                        }
                        None => {
                            if let Err(e) = parents.insert(entity, Parent::new(toolbox_entity)) {
                                warn!("toolbox parent system: Unable to associate parent: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}