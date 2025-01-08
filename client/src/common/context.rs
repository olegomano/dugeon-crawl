use std::marker::PhantomData;

extern crate file_manager;
extern crate input;
extern crate nalgebra;
extern crate slot_map;
extern crate sprite;
extern crate texture;

type SpriteManager = slot_map::SlotMap<sprite::Sprite>;
type TextureManager = file_manager::FileManager<texture::Texture>;

pub struct Context<'a, T> {
    texture_manager: TextureManager,
    sprites: SpriteManager,
    app_state: &'a mut T,
    player: slot_map::handle_t,
}

impl<'a, T> Context<'a, T> {
    pub fn new(state: &'a mut T) -> Self {
        return Self {
            texture_manager: TextureManager::new(file_manager::Config {
                max_bytes: 0,
                max_files: 0,
            }),
            sprites: SpriteManager::new(),
            app_state: state,
            player: slot_map::handle_t::CreateNull(),
        };
    }

    pub fn Init<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let player_image = self
            .texture_manager
            .Handle("/home/oleg/Documents/Dev/DungeonCrawl/asset/mage.png");
        let sprite = sprite::Sprite::new(player_image);

        self.player = self.sprites.Insert(sprite);
        f(self.app_state);
    }

    pub fn Destroy<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(self.app_state);
    }

    pub fn TickRender<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T, &mut TextureManager, &mut SpriteManager),
    {
        f(self.app_state, &mut self.texture_manager, &mut self.sprites);
    }

    /**
     * Returns True if program should keep running
     * Returns False if we want to exit
     */
    pub fn TickInput<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(&mut T) -> Vec<input::Action>,
    {
        let input = f(self.app_state);
        for i in input {
            match i {
                input::Action::Move(dir) => self.HandleMove(dir),
                input::Action::Quit() => return false,
                _ => {}
            }
        }
        return true;
    }

    fn HandleMove(&mut self, input: nalgebra::Vector4<f32>) {
        let mut player = self.sprites.GetMut(self.player).expect("");
        player.Displace(input.x, input.y, input.z);
    }
}
