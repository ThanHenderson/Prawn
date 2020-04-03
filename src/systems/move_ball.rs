#![allow(warnings)]
use crate::prawn::Ball;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut locals, balls, time): Self::SystemData) {
        // delta timing
        for (local, ball) in (&mut locals, &balls).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}
