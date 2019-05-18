extern crate amethyst;

use amethyst::utils::scene::BasicScenePrefab;
use amethyst::{
    assets::{Loader, PrefabLoader, PrefabLoaderSystem, ProgressCounter, RonFormat},
    core::{nalgebra::Vector3, Transform, TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{
        Camera, DisplayConfig, DrawFlat, Material, MaterialDefaults, ObjFormat, Pipeline,
        PosNormTex, Projection, RenderBundle, Stage,
    },
    utils::application_root_dir,
};

struct TankAttack;

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

impl SimpleState for TankAttack {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.add_resource(0usize);

        initialise_camera(world);

        let mut progress = ProgressCounter::default();

        let (tank, mtl) = {
            let mat_defaults = world.read_resource::<MaterialDefaults>();
            let loader = world.read_resource::<Loader>();

            let mesh_storage = world.read_resource();
            let textures = &world.read_resource();

            let tank = loader.load(
                "assets/turret3.obj",
                ObjFormat,
                (),
                &mut progress,
                &mesh_storage,
            );
            let albedo = loader.load_from_data([0.0, 0.0, 1.0, 0.0].into(), (), textures);
            let mat = Material {
                albedo,
                ..mat_defaults.0.clone()
            };

            (tank, mat)
        };

        let prefab_handle = world.exec(|loader: PrefabLoader<MyPrefabData>| {
            loader.load("resources/prefab.ron", RonFormat, (), ())
        });

        let mut trans = Transform::default();
        trans.set_xyz(0.0, 0.0, 0.0);
        trans.set_scale(2.0, 2.0, 2.0);

        world
            .create_entity()
            .with(trans)
            .with(tank)
            .with(mtl)
            .with(prefab_handle)
            .build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
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

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 10.0, 10.0);
    // transform.rotate_global(Vector3::x_axis(), 0.87 * -1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.309, 2.0)))
        .with(transform)
        .build();
}
