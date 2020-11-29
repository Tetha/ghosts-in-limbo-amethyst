use amethyst::ecs::Entity;
use amethyst::ui::UiCreator;
use amethyst::{GameData, SimpleState, StateData};


#[derive(Default)]
pub struct MainMenuState {
    ui_root: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator| creator.create("main_menu/menu.ron", ())));
    }
}