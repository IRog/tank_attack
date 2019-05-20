extern crate amethyst;
mod tank_attack;
mod systems;

use crate::tank_attack::MyPrefabData;
use crate::tank_attack::TankAttack;
use amethyst::{
    assets::PrefabLoaderSystem,
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawShadedSeparate, Pipeline, RenderBundle, Stage},
};
use amethyst_gltf::{GltfSceneLoaderSystem};
use std::path::Path;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = Path::new("./resources/display_config.ron");
    let config = DisplayConfig::load(&path);
    let bindings_path = Path::new("./resources/input.ron");
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(bindings_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawShadedSeparate::new()),
    );

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "scene_loader", &[])
        .with(
            GltfSceneLoaderSystem::default(),
            "",
            &["scene_loader"], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_bundle(input_bundle)?
        .with(systems::MovementSystem, "movement_system", &["input_system"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new("./", TankAttack, game_data)?;

    game.run();

    Ok(())
}
