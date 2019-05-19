use amethyst_gltf::{GltfSceneFormat, GltfSceneOptions};

use amethyst::{
    assets::{Loader, ProgressCounter},
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, Projection},
};

pub struct TankAttack;

impl SimpleState for TankAttack {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        initialise_camera(world);
        initialize_tank(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 13.0, -10.0);
    transform.rotate_global(Vector3::x_axis(), 0.67 * -1.0);
    transform.rotate_global(Vector3::y_axis(), 3.14159);

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.309, 2.0)))
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


    let mut trans = Transform::default();
    trans.set_xyz(0.0, 0.0, 0.0);
    trans.set_scale(2.0, 2.0, 2.0);

    world
        .create_entity()
        .with(trans)
        .with(asset)
        .build();
}
