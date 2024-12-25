extern crate nalgebra;
extern crate texture;
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
    pub src_rect: Matrix4<f32>,
    pub dst_rect: Matrix4<f32>,
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

    pub fn Blit(&self, input: &texture::Texture, output: &mut texture::Texture) {
        let screen_transform = matrix![
            1.0, 0.0,0.0,0.0;
            0.0,-1.0,0.0,1.0;
            0.0, 0.0,1.0,0.0;
            0.0, 0.0,0.0,1.0;
        ];

        let screen_to_output =
            screen_transform.try_inverse().expect("") * self.dst_rect.try_inverse().expect("");
        let output_to_screen = screen_to_output.try_inverse().expect("");

        let output_w_pixel = (self.dst_rect.column(0).xyz().norm() * output.width as f32);
        let output_h_pixel = (self.dst_rect.column(1).xyz().norm() * output.height as f32);
        let input_w_pixel = (self.src_rect.column(0).xyz().norm() * input.width as f32);
        let input_h_pixel = (self.src_rect.column(1).xyz().norm() * input.height as f32);

        let mut bottom_left = screen_to_output * Vector4::new(0.0, 0.0, 0.0, 1.0);
        let mut bottom_right = screen_to_output * Vector4::new(0.0, 1.0, 0.0, 1.0);
        let mut top_right = screen_to_output * Vector4::new(1.0, 1.0, 0.0, 1.0);
        let mut top_left = screen_to_output * Vector4::new(1.0, 0.0, 0.0, 1.0);
        Self::ClampVec(&mut bottom_left, 0.0, 1.0);
        Self::ClampVec(&mut top_right, 0.0, 1.0);
        Self::ClampVec(&mut bottom_right, 0.0, 1.0);
        Self::ClampVec(&mut top_left, 0.0, 1.0);

        let start_x = (Self::Min(bottom_left.x, bottom_right.x, top_right.x, top_left.x)
            * output_w_pixel) as u16;
        let end_x = (Self::Max(bottom_left.x, bottom_right.x, top_right.x, top_left.x)
            * output_w_pixel) as u16;
        let start_y = (Self::Min(bottom_left.y, bottom_right.y, top_right.y, top_left.y)
            * output_h_pixel) as u16;
        let end_y = (Self::Max(bottom_left.y, bottom_right.y, top_right.y, top_left.y)
            * output_h_pixel) as u16;

        for x_s in (start_x..end_x) {
            for y_s in (start_y..end_y) {
                let norm_x = (x_s - start_x) as f32 / (end_x - start_x) as f32;
                let norm_y = (y_s - start_y) as f32 / (end_y - start_y) as f32;

                let pixel_space = output_to_screen * Vector4::new(norm_x, norm_y, 0.0, 1.0);

                let pixel_x = pixel_space.x * input_w_pixel;
                let pixel_y = input_h_pixel - pixel_space.y * input_h_pixel;

                if pixel_x > 0.0 && pixel_y > 0.0 {
                    let color = input.Sample(pixel_x as u16, pixel_y as u16);
                    output.SetPixel(x_s, y_s, color);
                } else {
                    output.SetPixel(
                        x_s,
                        y_s,
                        texture::Pixel {
                            r: 255,
                            g: 0,
                            b: 0,
                            a: 0,
                        },
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
        Self::Clamp(&mut vector.x, min, max);
        Self::Clamp(&mut vector.y, min, max);
        Self::Clamp(&mut vector.z, min, max);
    }

    fn Clamp(out: &mut f32, min: f32, max: f32) {
        if *out > max {
            *out = max;
        } else if *out < min {
            *out = min;
        }
    }
}
