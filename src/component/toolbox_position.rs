use amethyst::ecs::{Component, DenseVecStorage};


#[derive(Debug)]
pub struct ToolboxPosition {
    pub index: usize
}

impl Component for ToolboxPosition {
    type Storage = DenseVecStorage<Self>;
}