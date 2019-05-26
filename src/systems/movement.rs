use crate::tank_attack::{Tank, TankCamera};
use amethyst::core::{nalgebra::Vector3, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct MovementSystem;

pub const MOVEMENT_SCALAR: f32 = 0.50;
pub const ROTATION_SCALAR: f32 = 0.08;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Tank>,
        ReadStorage<'s, TankCamera>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, tank, camera, input): Self::SystemData) {
        let rotation = input.axis_value("rotate_sideways").unwrap();
        let movement = input.axis_value("move_tank").unwrap();
        let mut tank_translation = Vector3::new(0.0, 0.0, 0.0);

        for (_, transform) in (&tank, &mut transforms).join() {
            transform.yaw_global(rotation as f32 * ROTATION_SCALAR);
            transform.move_backward(movement as f32 * MOVEMENT_SCALAR);
            tank_translation = *transform.translation();
        }

        for (_, transform) in (&camera, &mut transforms).join() {
            transform.yaw_global(rotation as f32 * ROTATION_SCALAR);
            transform.set_x(tank_translation.x as f32);
            transform.set_z(tank_translation.z as f32);
            transform.move_backward(5.0);
            transform.move_forward(movement as f32 * MOVEMENT_SCALAR);
        }
    }
}
