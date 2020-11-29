use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans, input};

use crate::main_menu::MainMenuState;

#[derive(Default)]
pub struct TileTestState;

impl SimpleState for TileTestState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Hello new state");
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if input::is_key_down(&event, input::VirtualKeyCode::Escape) {
                    return Trans::Switch(Box::new(MainMenuState::default()))
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
}