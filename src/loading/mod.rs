use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::{GameData, SimpleState, StateData};
use amethyst::assets::{AssetStorage, Handle, JsonFormat, Loader, ProgressCounter, RonFormat};

use crate::level::{WorldDefinition, WorldDefinitionHandle};
use crate::main_menu::MainMenuState;


#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    spritesheet_handle: Option<Handle<SpriteSheet>>,
    world_definition: Option<WorldDefinitionHandle>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        print!("Starting to load");
        //world.insert(AssetStorage::<WorldDefinition>::new());

        {
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

        let world_definition: Option<WorldDefinitionHandle>;
        {

            let loader = world.read_resource::<Loader>();
            world_definition = Some(loader.load(
                "world_definition.json",
                JsonFormat,
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<WorldDefinition>>()
            ));
        }
        world.insert(world_definition.unwrap());
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            println!("done loading");
            Trans::Switch(Box::new(
                MainMenuState::new(self.spritesheet_handle.take().expect("missing loaded spritesheet"))
            ))
        } else {
            if self.progress_counter.num_failed() > 0 {
                println!("Errors occured during asset loading. Aborting");
                for error in self.progress_counter.errors() {
                    println!("\t {} - {}", error.asset_name, error.error);
                }
                Trans::Quit
            } else {
                Trans::None
            }
        }
    }
}