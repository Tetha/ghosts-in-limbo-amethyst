use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::{GameData, SimpleState, StateData};
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter};

use crate::main_menu::MainMenuState;


#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    spritesheet_handle: Option<Handle<SpriteSheet>>
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let loader = world.read_resource::<Loader>();

        let texture = loader.load(
            "tiles.png", 
            ImageFormat::default(), 
            &mut self.progress_counter, 
            &world.read_resource::<AssetStorage<Texture>>()
        );
        
        self.spritesheet_handle = Some(loader.load(
            "tiles.ron",
            SpriteSheetFormat(texture),
            &mut self.progress_counter,
            &world.read_resource::<AssetStorage<SpriteSheet>>()
        ));
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            println!("done loading");
            Trans::Switch(Box::new(
                MainMenuState::new(self.spritesheet_handle.take().expect("missing loaded spritesheet"))
            ))
        } else {
            println!("waiting for load...");
            Trans::None
        }
    }
}