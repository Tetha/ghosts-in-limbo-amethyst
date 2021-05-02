/*
use amethyst::{renderer::SpriteRender, shred::System};
use amethyst::ecs::Join;
use amethyst::ecs::{ReadStorage, WriteStorage};
use crate::{component::SimpleArrowTile};


pub struct ArrowTileRenderSystem;

impl<'s> System<'s> for ArrowTileRenderSystem {
    type SystemData = (
        ReadStorage<'s, SimpleArrowTile>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (simple_arrows, mut sprite_renders): Self::SystemData) {
        for (simple_arrow, render) in (&simple_arrows, (&mut sprite_renders).maybe()).join() {
            match render {
                Some(_) => continue,
                None => {

                }
            }
        }
    }
}*/