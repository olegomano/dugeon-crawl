extern crate file_manager;
extern crate input;
extern crate nalgebra;
extern crate slot_map;
extern crate sprite;
extern crate texture;
extern crate transform;

type SpriteManager = slot_map::SlotMap<sprite::Sprite>;
type TextureManager = file_manager::FileManager<texture::Texture>;

pub struct GameContext {
    pub sprite_manager: SpriteManager,
    pub texture_manager: TextureManager,
}

pub trait IPlatform {
    fn OnInit(&mut self);
    fn OnRender(&mut self, context: &mut GameContext);
    fn OnDestroy(&mut self);
    fn GetInput(&mut self) -> Vec<input::Action>;
}

pub trait IMoveable {
    fn MoveTo(&self, x: f32, y: f32, z: f32);
    fn DisplaceBy(&self, dx: f32, dy: f32, dz: f32);
    fn LookAt(&self, x: f32, y: f32, z: f32);
    fn RotateBy(&self, da: f32);
}

pub trait IScene {
    fn OnInit(&mut self, context: &mut GameContext);
    fn OnTick(&mut self, context: &mut GameContext);
    fn OnInput(&mut self, context: &mut GameContext, input: &input::Action);
}
