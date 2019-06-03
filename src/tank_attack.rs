use amethyst::{
    assets::{Loader, PrefabLoader, ProgressCounter, RonFormat},
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, PosNormTex, Projection},
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

        initialize_ground(world);
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

    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 0.0);
    transform.set_scale(2.0, 2.0, 2.0);

    world
        .create_entity()
        .with(prefab_handle)
        .build();

    world
        .create_entity()
        .with(transform)
        .with(asset)
        .with(Tank::new())
        .build();
}

fn initialize_ground(world: &mut World) {
    let ground = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();

        loader.load("assets/hex_tile.glb", GltfSceneFormat, GltfSceneOptions::default(), &mut progress, &mesh_storage)
    };

    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(transform)
        .with(ground)
        .build();
}
