extern crate file_manager;
extern crate slot_map;
extern crate sprite;
extern crate texture;

type SpriteManager = slot_map::SlotMap<sprite::Sprite>;
type TextureManager = file_manager::FileManager<texture::Texture>;

pub struct Context<T> {
    texture_manager: TextureManager,
    sprites: SpriteManager,
    app_state: &mut T,
}

impl<T> Context<T> {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn Sprites(&self) -> &SpriteManager {
        return &self.sprites;
    }

    pub fn SpritesMut(&mut self) -> &mut SpriteManager {
        return &mut self.sprites;
    }

    pub fn RenderStep<T>(&mut self, f: T)
    where
        T: FnOnce(&mut SpriteManger, &mut TextureManager, &mut T),
    {
        f(
            &mut self.sprites,
            &mut self.texture_manager,
            &mut self.app_state,
        );
    }
}
