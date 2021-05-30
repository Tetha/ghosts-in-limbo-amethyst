use amethyst::{GameData, SimpleState, StateData, assets::PrefabLoader, prelude::{Builder, WorldExt}};
use amethyst::assets::RonFormat;
use amethyst::core::{Transform};
use amethyst::renderer::{Camera};
use crate::component::{InitialGhostPositionPrefab, StaticGridTilePrefab, Toolbox};

use super::LevelMetadata;


pub struct LevelState {
    level: LevelMetadata,
}

impl LevelState {
    pub fn new(level: LevelMetadata) -> LevelState {
        LevelState{ level }
    }
}

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let arrows_prefab = data.world.exec(|loader: PrefabLoader<'_, StaticGridTilePrefab>| {
            loader.load(
                format!("level/{}/static_grid_tiles.ron", self.level.id),
                RonFormat,
                (),
            )
        });
        let ghosts_prefab = data.world.exec(|loader: PrefabLoader<'_, InitialGhostPositionPrefab>| {
            loader.load(
                format!("level/{}/ghosts.ron", self.level.id),
                RonFormat,
                ()
            )
        });

        let foo = data.world
            .create_entity()
            .with(arrows_prefab.clone())
            .build();

        data.world
            .create_entity()
            .with(ghosts_prefab.clone())
            .build();


        let mut toolbox_transform = Transform::default();
        toolbox_transform.set_translation_xyz(500.0, 100., 0.);
        data.world
            .create_entity()
            .with(Toolbox{
                id: 0,
                capacity: 5,
                offset: 0 
            })
            .with(toolbox_transform)
            .build();
        let mut camera_transform = Transform::default();
        camera_transform.set_translation_xyz(512., 384., 1.0);
        data.world.create_entity()
                  .with(Camera::standard_2d(1024., 768.))
                  .with(camera_transform)
                  .build();
        println!("{:?}", foo);
        // TODO: now we need to add a system to add the appropriate transforms and such to draw stuff.
    }
}