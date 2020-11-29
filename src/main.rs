use amethyst::assets::HotReloadBundle;
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::{RenderToWindow, RenderingBundle};
use amethyst::renderer::types::DefaultBackend;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{Application, GameDataBuilder, Result};
use amethyst::utils::application_root_dir;

mod main_menu;

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

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
                .with_plugin(RenderUi::default()),
            // If you want to draw Sprites and such, you would need this additionally:
            // .with_plugin(RenderFlat2D::default())
        )?
        ;

    let mut game = Application::new(
        assets_dir,
        main_menu::MainMenuState::default(),
        game_data,
    )?;
    game.run();
    Ok(())
}
