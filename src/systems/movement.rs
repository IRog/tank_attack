use crate::tank_attack::{Tank, TankCamera};
use amethyst::core::{nalgebra::Vector3, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct MovementSystem;

pub const MOVEMENT_SCALAR: f32 = 1.4;
pub const ROTATION_SCALAR: f32 = 0.25;

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

        for (_, transform) in (&tank, &mut transforms).join() {
            transform.rotate_local(Vector3::y_axis(), rotation as f32 * ROTATION_SCALAR);
            transform.move_backward(movement as f32 * MOVEMENT_SCALAR);
        }

        for (_, transform) in (&camera, &mut transforms).join() {
            transform.rotate_local(Vector3::y_axis(), rotation as f32 * ROTATION_SCALAR);
            transform.move_forward(movement as f32 * MOVEMENT_SCALAR);
        }
    }
}
