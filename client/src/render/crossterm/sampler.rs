extern crate nalgebra;
extern crate texture;
extern crate transform;
use nalgebra::matrix;
use nalgebra::Matrix4;
use nalgebra::Vector4;

pub struct TextureBlitter<'a> {
    pub src_rect: &'a transform::Rect,
    pub src_texture: &'a texture::Texture,
    pub dst_rect: &'a transform::Rect,
    pub dst_texture: &'a mut texture::Texture,
}

impl TextureBlitter<'_> {
    pub fn Blit(&mut self) {
        let screen_space_norm = self.dst_rect.BoundingRect();

        let pixel_x_start =
            (screen_space_norm.BottomLeft().x * self.dst_texture.width as f32) as u16;
        let pixel_y_start =
            (screen_space_norm.BottomLeft().y * self.dst_texture.height as f32) as u16;
        let pixel_width = (screen_space_norm.Dims().0 * self.dst_texture.width as f32) as u16;
        let pixel_height = (screen_space_norm.Dims().1 * self.dst_texture.height as f32) as u16;

        for p_x in (pixel_x_start..(pixel_x_start + pixel_width)) {
            for p_y in (pixel_y_start..(pixel_y_start + pixel_height)) {
                //x,y in normalized pixel coordinates of dst texture space
                let n_x = p_x as f32 / pixel_width as f32;
                let n_y = p_y as f32 / pixel_height as f32;

                //project into the dst_rect to know the uv coordinates
                let uv = self
                    .dst_rect
                    .GlobalToLocal(Vector4::new(n_x, n_y, 0.0, 1.0));

                //check if we are out of bounds of the texture rect
                if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
                    continue;
                }

                let texture_sample = self.src_rect.LocalToGlobal(uv);
                let pixel_value = self
                    .src_texture
                    .SampleNorm(texture_sample.x, texture_sample.y);

                self.dst_texture
                    .SetPixelNorm(uv.x, uv.y, pixel_value, texture::BlendMode::ALPHA);
            }
        }
    }
}
