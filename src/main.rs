extern crate amethyst;
mod tank_attack;

use crate::tank_attack::MyPrefabData;
use crate::tank_attack::TankAttack;
use amethyst::{
    assets::PrefabLoaderSystem,
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawShaded, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);
    // let bindings_path = format!("{}/resources/input.ron", application_root_dir());
    // let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(bindings_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new("./", TankAttack, game_data)?;

    game.run();

    Ok(())
}
