use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;
use amethyst::ecs::{Entity, ReadStorage, WriteStorage};
use amethyst::input;
use amethyst::renderer::SpriteSheet;
use amethyst::shred::{ReadExpect, Write};
use amethyst::ui::{Anchor, FontAsset, Selectable, TtfFormat, UiButton, UiButtonBuilder, UiCreator, Widgets};
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use input::VirtualKeyCode;

use crate::component::LevelAssociation;
use crate::level::{LevelState, WorldDefinition};
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
        let StateData { world, .. } = data;
        
        world.register::<Selectable<String>>();
        world.insert(Widgets::<UiButton, String>::default());

        world.exec(|mut creator: UiCreator| {
            self.ui_root = Some(creator.create("main_menu/menu.ron", ()));
        });

        let world_definition = world.exec(|(world_def_assets, current_world) : (
            ReadExpect<AssetStorage<WorldDefinition>>,
            ReadExpect<Handle<WorldDefinition>>,
        )| {
            world_def_assets.get(&current_world).unwrap().clone()
        });
        
        let font_handle = world.exec(|(loader, font_store): (ReadExpect<Loader>, Write<AssetStorage<FontAsset>>)| {
            loader.load("font/square.ttf", TtfFormat, (), &font_store)
        });
        println!("{:?}", world_definition);
        for (i, level_definition) in world_definition.levels.iter().take(5).enumerate() {
            let (_, button) = UiButtonBuilder::<String, _>::new(level_definition.name.clone())
                                .with_id(format!("level_{}", i))
                                .with_parent(self.ui_root.unwrap())
                                .with_font(font_handle.clone())
                                //.with_text_color([1.0, 0.65, 0., 1.0])
                                .with_position(50., (200 + 100 * i) as f32)
                                .with_size(550., 50.)
                                .with_anchor(Anchor::BottomMiddle)
                                .build_from_world(&world);

            world.exec(|mut level_associations: WriteStorage<LevelAssociation>| {
                level_associations.insert(button.image_entity, LevelAssociation{
                    level_index: i,
                    level_definition: level_definition.clone(),
                }).expect("Cannot add component to ui button");
            })
            /*
                                world.create_entity()
                 .with(Parent::new(self.ui_root.unwrap()))
                 .with(button_transform)
                 .with(UiText::new(
                    font_handle.clone(),
                    level_definition.name.clone(),
                    [1.0, 0.65, 0., 1.0],
                    40.,
                    amethyst::ui::LineMode::Single,
                    Anchor::Middle,
                 ))
                 .with(LevelAssociation{level_index: i, level_definition: level_definition.clone()})
                 .build(); */
        };

    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ui_root) = self.ui_root {
            data.world.delete_entity(ui_root).expect("unable to remove main menu entity");
        }
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
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
            },
            StateEvent::Ui(event) => {
                world.exec(|level_association: ReadStorage<LevelAssociation>| {
                    match event.event_type {
                        amethyst::ui::UiEventType::Click => {
                            let level_association = level_association.get(event.target);
                            if let Some(assoc) = level_association {
                                println!("Level selected: {:?}", assoc);
                                Trans::Switch(Box::new(LevelState::new(assoc.level_definition.clone())))
                            } else {
                                Trans::None
                            }
                        },
                        _ => Trans::None  // ignore
                    }
                })
            },
            _ => Trans::None // ignore everything else
       } 
    }
}