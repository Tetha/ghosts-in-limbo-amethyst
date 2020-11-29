use amethyst::assets::Handle;
use amethyst::core::{Parent, Transform};
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, SpriteSheet};
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans, input};

use crate::game;
use crate::main_menu::MainMenuState;

pub struct TileTestState {
    sprite_sheet: Handle<SpriteSheet>,
    container_entity: Option<Entity>,
}

impl TileTestState {
    pub fn new(sprite_sheet: Handle<SpriteSheet>) -> TileTestState {
        TileTestState{sprite_sheet: sprite_sheet, container_entity: None}
    }
}
impl SimpleState for TileTestState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Hello new state");
        self.container_entity = Some(data.world.create_entity().build());

        let mut camera_transform = Transform::default();
        camera_transform.set_translation_xyz(512., 384., 1.0);
        data.world.create_entity()
                  .with(Camera::standard_2d(1024., 768.))
                  .with(camera_transform)
                  .with(Parent::new(self.container_entity.unwrap()))
                  .build();

        let mut arrow_transform = Transform::default();
        arrow_transform.set_translation_xyz(500., 500., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightUp, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightDown, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightUp, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightDown, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightLeft, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightRight, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightLeft, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.append_translation_xyz(50., 0., 0.);
        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionStraightRight, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.set_translation_xyz(500., 600., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionLeftTurnFromBottom, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionLeftTurnFromRight, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionLeftTurnFromTop, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionLeftTurnFromLeft, 
            arrow_transform.clone(), 
            self.container_entity);

        arrow_transform.set_translation_xyz(500., 700., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionRightTurnFromBottom, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionRightTurnFromRight, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionRightTurnFromTop, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);

        game::create_arrow_entity(
            data.world,
            self.sprite_sheet.clone(),
            game::ArrowDirection::ArrowDirectionRightTurnFromLeft, 
            arrow_transform.clone(), 
            self.container_entity);
        arrow_transform.append_translation_xyz(50., 0., 0.);
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