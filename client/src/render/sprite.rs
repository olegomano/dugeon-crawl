extern crate file_manager;
extern crate nalgebra;
extern crate transform;

use file_manager::handle_t;
use nalgebra::Matrix4;
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct TextureSample {
    pub texture: handle_t,
    pub src_rect: transform::Rect,
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub trans: transform::Transform,
    pub texture: TextureSample,
}

impl Sprite {
    pub fn new(image: handle_t) -> Self {
        return Self {
            texture: TextureSample {
                texture: image,
                src_rect: transform::Rect {
                    left: 0.0,
                    bottom: 0.0,
                    top: 1.0,
                    right: 1.0,
                },
            },
            trans: transform::Transform::new(),
        };
    }

    pub fn UpdateSampleRect(&mut self, rect: &transform::Rect) {
        self.texture.src_rect = rect.clone();
    }
}
