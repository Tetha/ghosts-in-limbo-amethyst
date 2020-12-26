mod tile_entity;
mod t_junction;

pub use tile_entity::{ArrowDirection, create_arrow_entity};
pub use t_junction::{TJunctionExit, TJunctionDirection, TJunctionMemoryPlacement, create_junction_entity};

pub enum MemoryType {
    Love,
    Pet,
    Sun,
}