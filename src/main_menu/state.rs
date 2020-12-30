use amethyst::assets::{AssetStorage, Handle};
use amethyst::prelude::*;
use amethyst::ecs::Entity;
use amethyst::input;
use amethyst::renderer::SpriteSheet;
use amethyst::shred::{Read, ReadExpect};
use amethyst::ui::UiCreator;
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use input::VirtualKeyCode;

use crate::level::WorldDefinition;
use crate::tile_test::TileTestState;


pub struct MainMenuState {
    ui_root: Option<Entity>,
    spritesheet: Handle<SpriteSheet>,
}

impl MainMenuState {
    pub fn new(spritesheet: Handle<SpriteSheet>) -> Self {
        MainMenuState{
            ui_root: None,
            spritesheet: spritesheet,
        }
    }
}
impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        world.exec(|(mut creator, assets, world_definition): (
            UiCreator,
            Read<'_, AssetStorage<WorldDefinition>>,
            ReadExpect<'_, Handle<WorldDefinition>>,
        )| {
            self.ui_root = Some(creator.create("main_menu/menu.ron", ()));
            let world_definition = assets.get(&world_definition);
            println!("{:?}", world_definition);
        });
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
                    return Trans::Switch(Box::new(TileTestState::new(self.spritesheet.clone())));
                }
                Trans::None // otherwise, do nothing
            }
            _ => Trans::None // ignore everything else
       } 
    }
}