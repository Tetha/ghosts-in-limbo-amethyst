use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans, input};

use crate::main_menu::MainMenuState;

pub struct TileTestState {
    sprite_sheet: Handle<SpriteSheet>,
}

impl TileTestState {
    pub fn new(sprite_sheet: Handle<SpriteSheet>) -> TileTestState {
        TileTestState{sprite_sheet: sprite_sheet}
    }
}
impl SimpleState for TileTestState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Hello new state");
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if input::is_key_down(&event, input::VirtualKeyCode::Escape) {
                    return Trans::Switch(Box::new(MainMenuState::new(self.sprite_sheet.clone())))
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
}