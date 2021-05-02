use amethyst::ecs::{Component, NullStorage};


/*
 * Initial Ghost Position is used to flag the initial ghost positions.
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct InitialGhostPosition;

impl Component for InitialGhostPosition {
    type Storage = NullStorage<Self>;
}