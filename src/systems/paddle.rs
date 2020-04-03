use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::prawn::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// SystemDesc trait must be implemented for systems instantiation
#[derive(SystemDesc)]
pub struct PaddleSystem;

// Use the lifrtime of the components that the system operates on
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        // Mutate transform components
        WriteStorage<'s, Transform>,
        // Reads paddle components
        ReadStorage<'s, Paddle>,
        // Has access to Input handler that we use
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        // par_join can be used for parallel processing
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            // Get the amount of movement for the appropriate paddle
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            // If there is movement get the side that moved
            if let Some(mv_amount) = movement {
                // This can be tied to the time elapsed rather than the framerate with core::timing::Time
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + scaled_amount)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5),
                );
            }
        }
    }
}
