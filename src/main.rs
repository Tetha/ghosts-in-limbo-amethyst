use amethyst::assets::{HotReloadBundle, PrefabLoaderSystemDesc, Processor};
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
use amethyst::renderer::types::DefaultBackend;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{Application, GameDataBuilder, LoggerConfig, Result};
use amethyst::utils::application_root_dir;
use component::{ArrowTilePrefab, InitialGhostPositionPrefab};
use level::WorldDefinition;
use loading::LoadingState;
use systems::{GhostDirectionIndicatorUpdater, GridCoordinateTransformer};

mod component;

mod main_menu;
mod tile_test;
mod game;
mod loading;
mod level;
mod systems;

fn main() -> Result<()> {
    let mut log_config = LoggerConfig::default();
    log_config.level_filter = amethyst::LogLevelFilter::Debug;
    amethyst::start_logger(log_config);

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::default())?
        .with_bundle(InputBundle::<StringBindings>::new())? // TODO: replace string bindings
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
        )?
        .with(Processor::<WorldDefinition>::new(), "", &[])
        .with_system_desc(
            PrefabLoaderSystemDesc::<ArrowTilePrefab>::default(),
            "",
            &[])
        .with_system_desc(
            PrefabLoaderSystemDesc::<InitialGhostPositionPrefab>::default(),
            "",
            &[])
        .with(GridCoordinateTransformer, "", &[])
        .with(GhostDirectionIndicatorUpdater, "", &[])
        ;

    let mut game = Application::new(
        assets_dir,
        LoadingState::default(),
        game_data,
    )?;
    game.run();
    Ok(())
}
