use amethyst::{core::Transform, core::SystemDesc, ecs::{ReadStorage, WriteStorage}};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, System, SystemData};
use crate::component::GridPosition;

#[derive(SystemDesc)]
pub struct GridCoordinateTransformer;

impl<'a> System<'a> for GridCoordinateTransformer {
    type SystemData = (
        ReadStorage<'a, GridPosition>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (grid_positions, mut positions): Self::SystemData) {
        for (grid_position, position) in (&grid_positions, &mut positions).join() {
            position.set_translation_xyz((grid_position.x as f32) * 48., (grid_position.y as f32) * 48., 0.);
        }
    }
}