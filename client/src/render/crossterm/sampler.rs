extern crate nalgebra;
extern crate texture;
extern crate transform;
use nalgebra::matrix;
use nalgebra::Matrix4;
use nalgebra::Vector4;
/*
 * Represents a sampler that samples every pixel on dst_rect from src_rect
 * IE it blits src_rect into dst_rect
 *
 * the Matrix is interpreted as a transform over a 1x1 rect that is homogneous
 * coorinate system.
 *
 * IE a identity matrix represents blitting to the whole screen
 */
pub struct Sampler {
    pub src_rect: Rect,
    pub dst_rect: Rect,
}

/*
 * When sampling the output rect we return the screen space homogenous coordinate
 * And the color sample that corresponds to it
 *
 */
pub struct Sample {
    x: f32,
    y: f32,
    color: texture::Pixel,
}

impl Sampler {
    /*
     * 0,0 represents sampling the bottom left of the output rect
     * 1,1 represents sampling the top-right of the output rect
     */
    pub fn Sample(&self, x: f32, y: f32, img: &texture::Texture) -> Sample {
        //location in output image we want to sample in normalized coords
        let sample_location = Vector4::new(x, y, 0.0, 1.0);
        //the location in screen/output buffer in normalized coords
        let output_location = self.dst_rect * sample_location;
        //the location in the image we want to sample in noramlized coords
        let input_location = self.src_rect * sample_location;

        return Sample {
            x: output_location.x,
            y: output_location.y,
            color: img.SampleNorm(input_location.x, input_location.y),
        };
    }

    /*
     * We will blit from src_rect in the input texture into dst_rect in the output texture
     */
    pub fn Blit(&self, input: &texture::Texture, output: &mut texture::Texture) {
        let w_h_ratio = output.height as f32 / output.width as f32;

        let screen_transform = matrix![
            1.0, 0.0,0.0,0.0;
            0.0,-1.0 * w_h_ratio,0.0,0.0;
            0.0, 0.0,2.0,0.0;
            0.0, 0.0,0.0,1.0;
        ];
        //transforms from input_texture normalized coordinates into output_texture normalized
        //coordinates
        let screen_to_output = screen_transform.try_inverse().expect("") * self.dst_rect;
        let output_to_screen = screen_to_output.try_inverse().expect("");

        let bottom_left = screen_to_output * Vector4::new(-0.5, -0.5, 0.0, 1.0);
        let bottom_right = screen_to_output * Vector4::new(0.5, -0.5, 0.0, 1.0);
        let top_right = screen_to_output * Vector4::new(0.5, 0.5, 0.0, 1.0);
        let top_left = screen_to_output * Vector4::new(-0.5, 0.5, 0.0, 1.0);

        let start_x = (Self::Min(bottom_left.x, bottom_right.x, top_right.x, top_left.x)
            * output.width as f32) as i16;
        let end_x = (Self::Max(bottom_left.x, bottom_right.x, top_right.x, top_left.x)
            * output.width as f32) as i16;
        let start_y = (Self::Min(bottom_left.y, bottom_right.y, top_right.y, top_left.y)
            * output.height as f32) as i16;
        let end_y = (Self::Max(bottom_left.y, bottom_right.y, top_right.y, top_left.y)
            * output.height as f32) as i16;

        let mut screen_start_x = start_x;
        let mut screen_start_y = start_y;
        let mut screen_end_x = end_x;
        let mut screen_end_y = end_y;

        Self::Clamp::<i16>(&mut screen_start_x, 0, output.width as i16);
        Self::Clamp::<i16>(&mut screen_end_x, 0, output.width as i16);
        Self::Clamp::<i16>(&mut screen_start_y, 0, output.height as i16);
        Self::Clamp::<i16>(&mut screen_end_y, 0, output.height as i16);

        for x_s in (screen_start_x..screen_end_x) {
            for y_s in (screen_start_y..screen_end_y) {
                //normalized screen coordinate
                //let norm_x = (x_s - start_x) as f32 / (end_x - start_x) as f32;
                //let norm_y = (y_s - start_y) as f32 / (end_y - start_y) as f32;

                let norm_x = x_s as f32 / output.width as f32;
                let norm_y = y_s as f32 / output.height as f32;

                if norm_x < 0.0 || norm_x > 1.0 || norm_y < 0.0 || norm_y > 1.0 {
                    continue;
                }

                //texture coordinate
                let mut pixel_space =
                    output_to_screen * self.src_rect * Vector4::new(norm_x, norm_y, 0.0, 1.0);

                pixel_space.x = pixel_space.x + 0.5;
                pixel_space.y = pixel_space.y + 0.5;

                let pixel_x = pixel_space.x * input.width as f32;
                let pixel_y = pixel_space.y * input.height as f32;

                if pixel_space.x > 0.0
                    && pixel_space.y > 0.0
                    && pixel_space.x < 1.0
                    && pixel_space.y < 1.0
                {
                    let color = input.Sample(pixel_x as u16, pixel_y as u16);
                    output.SetPixel(x_s as u16, y_s as u16, color, texture::BlendMode::ALPHA);
                } else {
                    output.SetPixel(
                        x_s as u16,
                        y_s as u16,
                        texture::Pixel {
                            r: 255,
                            g: 0,
                            b: 0,
                            a: 0,
                        },
                        texture::BlendMode::ALPHA,
                    );
                }
            }
        }
    }

    fn Min(a: f32, b: f32, c: f32, d: f32) -> f32 {
        return a.min(b).min(c).min(d);
    }

    fn Max(a: f32, b: f32, c: f32, d: f32) -> f32 {
        return a.max(b).max(c).max(d);
    }

    fn ClampVec(vector: &mut Vector4<f32>, min: f32, max: f32) {
        Self::Clamp::<f32>(&mut vector.x, min, max);
        Self::Clamp::<f32>(&mut vector.y, min, max);
        Self::Clamp::<f32>(&mut vector.z, min, max);
    }

    fn Clamp<T: std::cmp::PartialOrd>(out: &mut T, min: T, max: T) {
        if *out > max {
            *out = max;
        } else if *out < min {
            *out = min;
        }
    }
}

pub struct TextureBlitter {
    pub src_rect: &transform::Rect,
    pub src_texture: &texture::Texture,
    pub dst_rect: &transform::Rect,
    pub dst_texture: &mut transform::Rect,
}

impl TextureBlitter {
    pub fn Blit(&self) {
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
                    .SetPixelNorm(uv.x, uv.y, pixel, texture::BlendMode::ALPHA);
            }
        }
    }
}
