extern crate context;
extern crate file_manager;
extern crate input;
extern crate sprite_sheet;
extern crate transform;
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
pub struct SheetConfig {
    pub cell_count_w: u8,
    pub cell_count_h: u8,
    pub image_handle: file_manager::handle_t,
}

/*
 * Has two transforms, one is the "logical traansform" for game logic
 * The other is the render transofmr, IE where in world space should the ojbect be rendered
 *
 */
pub struct CharController {
    pos_transform: transform::Transform,
    sprite_sheet_config: Option<SheetConfig>,
    sprite_sheet: Option<sprite_sheet::SpriteSheet>,
    texture_handle: file_manager::handle_t,
    sprite: slot_map::handle_t,
}

impl CharController {
    pub fn new_null() -> Self {
        return Self {
            pos_transform: transform::Transform::new(),
            texture_handle: file_manager::handle_t::new_null(),
            sprite_sheet: None,
            sprite_sheet_config: None,
            sprite: slot_map::handle_t::CreateNull(),
        };
    }

    /*
     * Creates a character controller of a sprite type of image
     * This has implications for how rotations are handled, since instead of
     * rotating the whole image we render the index of the sprite sheet correspoding
     * to the direction
     */
    pub fn new_sprite(sheet_config: &SheetConfig) -> Self {
        return Self {
            pos_transform: transform::Transform::new(),
            texture_handle: sheet_config.image_handle.clone(),
            sprite_sheet_config: Some(sheet_config.clone()),
            sprite_sheet: None,
            sprite: slot_map::handle_t::CreateNull(),
        };
    }

    pub fn new_image(texture: file_manager::handle_t) -> Self {
        return Self {
            pos_transform: transform::Transform::new(),
            texture_handle: texture.clone(),
            sprite_sheet: None,
            sprite_sheet_config: None,
            sprite: slot_map::handle_t::CreateNull(),
        };
    }

    pub fn Init(&mut self, context: &mut game_object::GameContext) {
        self.sprite = context
            .sprite_manager
            .Insert(sprite::Sprite::new(self.texture_handle));

        if let Some(sprite_sheet_config) = self.sprite_sheet_config {
            let image = context.texture_manager.Load(self.texture_handle);
            self.sprite_sheet = Some(sprite_sheet::SpriteSheet::FromRegularGrid(
                sprite_sheet_config.cell_count_w,
                sprite_sheet_config.cell_count_h,
                image.width as u32,
                image.height as u32,
            ));
        }
    }

    pub fn Destroy(&mut self, context: &mut game_object::GameContext) {
        context.sprite_manager.Erase(self.sprite);
    }

    pub fn HandleAction(&mut self, action: &input::Action, context: &mut game_object::GameContext) {
        match action {
            input::Action::Move(dir) => {
                self.pos_transform.Displace(dir.x, dir.y, dir.z);
                if let Some(sprite) = context.sprite_manager.GetMut(self.sprite) {
                    sprite.trans.Displace(dir.x, dir.y, dir.z);
                };
            }
            input::Action::Rotate(angle) => {
                self.pos_transform.RotateOriginBy(*angle);
                if let Some(sprite_sheet) = &self.sprite_sheet {
                    //handle throuugh sprite sheet
                } else {
                    if let Some(sprite) = context.sprite_manager.GetMut(self.sprite) {
                        sprite.trans.RotateOriginBy(*angle);
                    };
                }
            }
            _ => {}
        }
    }
}
