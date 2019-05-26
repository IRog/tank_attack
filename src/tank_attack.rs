use amethyst::{
    assets::{AssetStorage, Loader, PrefabLoader, ProgressCounter, RonFormat},
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, PngFormat, PosNormTex, Projection, Texture, TextureMetadata},
};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::utils::scene::BasicScenePrefab;
use amethyst_gltf::{GltfSceneFormat, GltfSceneOptions};

pub type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;
pub struct TankAttack;
pub struct Tank {}
pub struct TankCamera {}

impl Tank {
    fn new() -> Tank {
        Tank {}
    }
}

impl TankCamera {
    fn new () -> TankCamera {
        TankCamera {}
    }
}

impl Component for Tank {
    type Storage = DenseVecStorage<Self>;
}

impl Component for TankCamera {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for TankAttack {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        initialize_image(world);
        initialise_camera(world);
        initialize_tank(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 7.0, -5.0);
    transform.rotate_global(Vector3::y_axis(), 3.14159);

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.309, 2.0)))
        .with(TankCamera::new())
        .with(transform)
        .build();
}

fn initialize_tank(world: &mut World) {
    let asset = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();

        loader.load("assets/turret3.glb", GltfSceneFormat, GltfSceneOptions::default(), &mut progress, &mesh_storage)
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
        .with(asset)
        .with(prefab_handle)
        .with(Tank::new())
        .build();
}

fn initialize_image(world: &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "assets/metal.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };

    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 0.0);
    // transform.rotate_global(Vector3::y_axis(), 90.0_f32.to_radians());

    world
        .create_entity()
        .with(transform)
        .with(texture_handle)
        .build();
}
