extern crate amethyst;
mod tank_attack;

use crate::tank_attack::MyPrefabData;
use crate::tank_attack::TankAttack;
use amethyst::{
    assets::PrefabLoaderSystem,
    core::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawShadedSeparate, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};
use amethyst_gltf::{GltfSceneLoaderSystem};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

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
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new("./", TankAttack, game_data)?;

    game.run();

    Ok(())
}
