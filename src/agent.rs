use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::world::Builder;
use amethyst::prelude::World;
use amethyst::renderer::{
    PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
    TextureMetadata,
};
use crate::{social::Social, movement::Movement};
use crate::WIDTH;
use crate::HEIGHT;
use rand::distributions::Uniform;
use rand::{Rng};

pub fn create_agent_at(x: f32, y: f32, world: &mut World, texture: &SpriteSheetHandle) {
    let mut transform = Transform::default();
    transform.translate_x(x);
    transform.translate_y(y);

    let sprite_render = SpriteRender {
        sprite_sheet: texture.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .with(Social::default())
        .with(Movement::default())
        .build();
}

pub fn create_agent_at_random_location(world: &mut World, texture: &SpriteSheetHandle) {
    let mut rng = rand::thread_rng();
    let x = rng.sample(Uniform::new(0.0, WIDTH));
    let y = rng.sample(Uniform::new(0.0, HEIGHT));
    create_agent_at(x, y, world, texture);
}

pub fn load_agent_texture(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "resources/texture/agent_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "resources/texture/agent_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_storage,
    )
}
