use amethyst::{ecs::{Component, VecStorage, System, Entities, WriteStorage, ReadStorage, Join}, core::Transform};
use crate::movement::Movement;
use std::mem;

#[derive(Default)]
pub struct Social {
    like: Vec<f32>,
}

impl Component for Social {
    type Storage = VecStorage<Self>;
}

pub struct SocialSystem;

const DEFAULT_DISTANCE: f32 = 10.0;
const LIKE_DAMPENING: f32 = 0.00000001;
const LIKE_UPDATE: f32 = 0.0001;
const LIKE_MOVEMENT: f32 = 0.001;

impl<'s> System<'s> for SocialSystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, Social>, ReadStorage<'s, Transform>, WriteStorage<'s, Movement>);

    fn run(&mut self, (entities, mut socials, transforms, mut movements): <Self as System<'s>>::SystemData) {
        // Clear movements
        for (mut movement, _) in (&mut movements, &socials).join() {
            movement.x = 0.0;
            movement.y = 0.0;
        }

        for entity in entities.join() {
            if socials.contains(entity) && transforms.contains(entity) {
                // Calculate distance vector
                let t1 = transforms.get(entity).unwrap().translation();

                // Initialize likes if not initialized
                let mut social = socials.get_mut(entity).unwrap();
                if social.like.len() == 0 {
                    let amount = (&socials, &transforms, &movements).join().count();
                    social = socials.get_mut(entity).unwrap();
                    social.like.resize(amount, 0.0);
                }

                // Swap out like vector
                let mut like = Vec::new();
                mem::swap(&mut like, &mut social.like);
                let mut movement = Movement::default();

                for (like, (other_entity, _social, transform, _movement)) in like.iter_mut().zip((&entities, &socials, &transforms, &mut movements).join()) {
                    if entity == other_entity {
                        continue;
                    }

                    let t2 = transform.translation();
                    let dx = t2.x - t1.x;
                    let dy = t2.y - t1.y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance > 1e-4 {
                        let movement_factor = update_like(like, distance);
                        movement.x += dx / distance * movement_factor;
                        movement.y += dy / distance * movement_factor;
                    }
                }

                // Swap like vector back
                let social = socials.get_mut(entity).unwrap();
                mem::swap(&mut like, &mut social.like);
                *movements.get_mut(entity).unwrap() = movement;
            }
        }
    }
}

fn desired_distance(like: f32) -> f32 {
    DEFAULT_DISTANCE - like
}

fn update_like(like: &mut f32, distance: f32) -> f32 {
    let desired_distance = desired_distance(*like);
    let diff = distance - desired_distance;
    *like -= diff * LIKE_UPDATE;
    *like *= 1.0 - LIKE_DAMPENING;
    diff * LIKE_MOVEMENT
}