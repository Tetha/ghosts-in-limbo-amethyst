use amethyst::ecs::{Component, DenseVecStorage};


#[derive(Debug)]
pub struct ToolboxPosition {
    pub toolbox: usize,
    pub index: i32
}

impl Component for ToolboxPosition {
    type Storage = DenseVecStorage<Self>;
}