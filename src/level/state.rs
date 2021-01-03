use amethyst::SimpleState;

use super::LevelDefinition;


pub struct LevelState {
    level: LevelDefinition,
}

impl LevelState {
    pub fn new(level: LevelDefinition) -> LevelState {
        LevelState{ level }
    }
}

impl SimpleState for LevelState {

}