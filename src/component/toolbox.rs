use amethyst::ecs::{Component, DenseVecStorage};


#[derive(Debug)]
pub struct Toolbox {
    // used to place the right elements into this toolbox at load
    pub id: usize,

    // how many elements can this toolbox display?
    pub capacity: usize,

    // offset into the toolbox elements. Used for scrolling.
    pub offset: usize,
}

impl Component for Toolbox {
    type Storage = DenseVecStorage<Self>;
}