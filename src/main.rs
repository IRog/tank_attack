extern crate amethyst;

use amethyst::{
    assets::{Loader, ProgressCounter},
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

impl SimpleState for TankAttack {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("orion is cute");
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

        let mut trans = Transform::default();
        trans.set_xyz(0.0, 0.0, 0.0);
        trans.set_scale(2.0, 2.0, 2.0);

        world
            .create_entity()
            .with(trans)
            .with(tank)
            .with(mtl)
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
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new("./", TankAttack, game_data)?;

    game.run();

    Ok(())
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 12.5, 25.0);
    //transform.rotate_global(Vector3::x_axis(), 1.5708 * -1.50);
    //transform.rotate_local(Vector3::y_axis(), 1.5708 * -0.1);
//https://github.com/amethyst/ludumdare42/blob/master/amethyst/examples/asset_loading/main.rs
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(
            1.0,
            std::f32::consts::FRAC_PI_3,
        )))
        .with(transform)
        .build();
}
