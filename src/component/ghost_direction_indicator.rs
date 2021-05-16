use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Clone, Copy, Default)]
pub struct GhostDirectionIndicator;

impl Component for GhostDirectionIndicator {
    type Storage = NullStorage<Self>;
}