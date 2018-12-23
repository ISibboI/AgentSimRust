use amethyst::{ecs::{Component, VecStorage, System, ReadStorage, WriteStorage, Join},
core::Transform};
use crate::WIDTH;
use crate::HEIGHT;

#[derive(Default)]
pub struct Movement {
    pub x: f32,
    pub y: f32,
}

impl Component for Movement {
    type Storage = VecStorage<Self>;
}

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Movement>);

    fn run(&mut self, (mut transforms, movements): <Self as System<'a>>::SystemData) {
        for (transform, movement) in (&mut transforms, &movements).join() {
            transform.translate_x(movement.x);
            transform.translate_y(movement.y);

            if transform.translation()[0] > WIDTH {
                transform.translation_mut()[0] = WIDTH;
            }
            if transform.translation()[1] > HEIGHT {
                transform.translation_mut()[1] = HEIGHT;
            }
            if transform.translation()[0] < 0.0 {
                transform.translation_mut()[0] = 0.0;
            }
            if transform.translation()[1] < 0.0 {
                transform.translation_mut()[1] = 0.0;
            }
        }
    }
}