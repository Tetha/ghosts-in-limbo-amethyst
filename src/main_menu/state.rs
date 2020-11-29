use amethyst::prelude::*;
use amethyst::ecs::Entity;
use amethyst::input;
use amethyst::ui::UiCreator;
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use input::VirtualKeyCode;

use crate::tile_test::TileTestState;


#[derive(Default)]
pub struct MainMenuState {
    ui_root: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator| creator.create("main_menu/menu.ron", ())));
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ui_root) = self.ui_root {
            data.world.delete_entity(ui_root).expect("unable to remove main menu entity");
        }
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
       match event {
            StateEvent::Window(event) => {
                if input::is_close_requested(&event) {
                    return Trans::Quit;
                }
                if input::is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                }
                if input::is_key_down(&event, VirtualKeyCode::F1) {
                    return Trans::Switch(Box::new(TileTestState::default()));
                }
                Trans::None // otherwise, do nothing
            }
            _ => Trans::None // ignore everything else
       } 
    }
}