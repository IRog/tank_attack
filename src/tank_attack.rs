use amethyst_gltf::GltfSceneFormat;
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::{
    assets::{Loader},
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, PosNormTex, Projection},
};

pub struct TankAttack;

pub type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

// #[derive(PartialEq, Eq)]
// pub enum Side {
//     Left,
//     Right,
// }

// pub struct Paddle {
//     pub side: Side,
//     pub width: f32,
//     pub height: f32,
// }

// impl Paddle {
//     fn new(side: Side) -> Paddle {
//         Paddle {
//             side,
//             width: PADDLE_WIDTH,
//             height: PADDLE_HEIGHT,
//         }
//     }
// }

// impl Component for Paddle {
//     type Storage = DenseVecStorage<Self>;
// }

impl SimpleState for TankAttack {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        // world.add_resource(0usize);

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
    // let mut progress = ProgressCounter::default();

    // let (tank, mtl) = {
    //     let mat_defaults = world.read_resource::<MaterialDefaults>();
    //     let loader = world.read_resource::<Loader>();

    //     let mesh_storage = world.read_resource();
    //     let textures = &world.read_resource();

    //     let tank = loader.load(
    //         "assets/turret3.obj",
    //         ObjFormat,
    //         (),
    //         &mut progress,
    //         &mesh_storage,
    //     );
    //     // let albedo = loader.load_from_data([0.5, 0.5, 0.5, 0.5].into(), (), textures);
    //     let mat = Material {
    //         textures,
    //         ..mat_defaults.0.clone()
    //     };

    //     (tank, mat)
    // };

    let asset = {
        let loader = world.read_resource::<Loader>();
        loader.load("assets/turret3.gltf", GltfSceneFormat, Default::default(), (), &world.read_resource())
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
