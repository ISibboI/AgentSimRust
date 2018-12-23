extern crate amethyst;

use crate::agent::create_agent_at;
use crate::agent::load_agent_texture;
use amethyst::{
    core::transform::{Transform, TransformBundle},
    prelude::*,
    renderer::{
        Camera, DisplayConfig, DrawFlat2D, Pipeline, Projection, RenderBundle, Stage, SpriteRender,
    },
    utils::application_root_dir,
};
use crate::movement::MovementSystem;
use crate::social::SocialSystem;
use crate::agent::create_agent_at_random_location;

mod agent;
mod movement;
mod social;

const WIDTH: f32 = 100.0;
const HEIGHT: f32 = 100.0;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut camera_transform = Transform::default();
        camera_transform.set_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(0.0, WIDTH, 0.0, HEIGHT)))
            .with(camera_transform)
            .build();

        world.register::<SpriteRender>();
        let agent_texture = load_agent_texture(world);
        create_agent_at(WIDTH / 2.0 - 4.0, HEIGHT / 2.0, world, &agent_texture);
        create_agent_at(WIDTH / 2.0 + 4.0, HEIGHT / 2.0, world, &agent_texture);

        for _ in 0..10 {
            create_agent_at_random_location(world, &agent_texture);
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with(SocialSystem, "social_system", &[])
        .with(MovementSystem, "movement_system", &["social_system"]);
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
