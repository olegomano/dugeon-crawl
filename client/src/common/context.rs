extern crate file_manager;
extern crate game_object;
extern crate input;
extern crate nalgebra;
extern crate slot_map;
extern crate sprite;
extern crate texture;
use std::marker::PhantomData;

type SpriteManager = slot_map::SlotMap<sprite::Sprite>;
type TextureManager = file_manager::FileManager<texture::Texture>;

pub struct Context<P, S> {
    platform: P,
    scene: S,
    game_context: game_object::GameContext,
}

impl<P: game_object::IPlatform, S: game_object::IScene> Context<P, S> {
    pub fn new(platform: P, scene: S) -> Self {
        return Self {
            platform: platform,
            scene: scene,
            game_context: game_object::GameContext {
                sprite_manager: SpriteManager::new(),
                texture_manager: TextureManager::new(file_manager::Config {
                    max_bytes: 0,
                    max_files: 0,
                }),
            },
        };
    }

    pub fn Init(&mut self) {
        self.platform.OnInit();
        self.scene.OnInit(&mut self.game_context);
    }

    pub fn Destroy(&mut self) {
        self.platform.OnDestroy();
    }

    /*
     * Returns true if we should continue executing, returns false if we are done
     */
    pub fn Tick(&mut self) -> bool {
        let input = self.platform.GetInput();
        for i in input {
            match i {
                input::Action::Quit() => {
                    return false;
                }
                _ => {
                    self.scene.OnInput(&mut self.game_context, &i);
                }
            }
        }
        self.scene.OnTick(&mut self.game_context);
        self.platform.OnRender(&mut self.game_context);
        return true;
    }
}
