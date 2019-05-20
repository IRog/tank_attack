use crate::tank_attack::{Tank, TankCamera};
use amethyst::core::{Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct MovementSystem;

pub const MOVEMENT_SCALAR: f32 = 0.50;
pub const ROTATION_SCALAR: f32 = 0.10;

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
            transform.yaw_global(rotation as f32 * ROTATION_SCALAR);
            transform.move_backward(movement as f32 * MOVEMENT_SCALAR);
        }

        for (_, transform) in (&camera, &mut transforms).join() {
            if rotation != 0.0 {
                transform.yaw_local(rotation as f32 * ROTATION_SCALAR * 0.5);
                transform.move_right(rotation as f32 * 0.25);
            }
            transform.move_forward(movement as f32 * MOVEMENT_SCALAR);
        }
    }
}
