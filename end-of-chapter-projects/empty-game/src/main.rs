use amethyst::utils::application_root_dir;
use amethyst::SimpleState;
use amethyst::GameDataBuilder;
use amethyst::Application;
use amethyst::renderer::{
    plugins::{RenderShaded3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::window::DisplayConfig;

struct GameState;
impl SimpleState for GameState {}

fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Cubefield".to_string(),
        dimensions: Some((1024, 768)),
        ..Default::default()
    };

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.95, 0.95, 0.95, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderShaded3D::default()),
        )?;

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}