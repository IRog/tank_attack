use amethyst::utils::scene::BasicScenePrefab;
use amethyst::{
    assets::{Loader, PrefabLoader, ProgressCounter, RonFormat},
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, Material, MaterialDefaults, ObjFormat, PosNormTex, Projection},
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
    transform.set_xyz(0.0, 10.0, 10.0);
    // transform.rotate_global(Vector3::x_axis(), 0.87 * -1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.309, 2.0)))
        .with(transform)
        .build();
}

fn initialize_tank(world: &mut World) {
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

// fn initialize_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
//     let mut left_transform = Transform::default();
//     let mut right_transform = Transform::default();
//     let y = ARENA_HEIGHT / 2.0;

//     left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
//     right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

//     let sprite_render = SpriteRender {
//         sprite_sheet: sprite_sheet.clone(),
//         sprite_number: 0,
//     };

//     world
//         .create_entity()
//         .with(sprite_render.clone())
//         .with(Paddle::new(Side::Left))
//         .with(left_transform)
//         .build();

//     world
//         .create_entity()
//         .with(sprite_render.clone())
//         .with(Flipped::Horizontal)
//         .with(Paddle::new(Side::Right))
//         .with(right_transform)
//         .build();
// }
