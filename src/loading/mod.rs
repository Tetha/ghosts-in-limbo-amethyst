use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::{GameData, SimpleState, StateData};
use amethyst::assets::{AssetStorage, Handle, JsonFormat, Loader, ProgressCounter};

use crate::component::LevelAssociation;
use crate::level::WorldDefinition;
use crate::main_menu::MainMenuState;


#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    spritesheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        print!("Starting to load");
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

        let world_definition_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "world_definition.json",
                JsonFormat,
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<WorldDefinition>>()
            )
        };
        world.register::<LevelAssociation>(); // TODO: remove
        world.insert(world_definition_handle);
        world.insert(self.spritesheet_handle.clone().expect("expected spritesheet to exist"));
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