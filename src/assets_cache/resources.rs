use bevy::{
    prelude::{AssetServer, FromWorld, Handle, Resource},
    text::Font,
};

#[derive(Resource)]
pub struct AssetsCache {
    pub sprites: Sprites,
    pub fonts: Fonts,
}

pub struct Fonts {
    pub main: Handle<Font>,
}

pub struct Sprites {
    pub entities: EntitiesSprites,
    pub characters: CharactersSprites,
    pub ui: UISprites,
}

pub struct UISprites {}
pub struct EntitiesSprites {}
pub struct CharactersSprites {}

impl FromWorld for AssetsCache {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let assets_loader = world.get_resource::<AssetServer>().unwrap();

        return AssetsCache {
            fonts: Fonts {
                main: assets_loader.load("fonts/CyrillicPixel.ttf"),
            },
            sprites: Sprites {
                characters: CharactersSprites {},
                ui: UISprites {},
                entities: EntitiesSprites {},
            },
        };
    }
}
