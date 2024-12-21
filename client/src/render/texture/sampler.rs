extern crate nalgebra;
extern crate texture;
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
        let sample_count = 512;

        for x_s in 0..sample_count {
            for y_s in 0..sample_count {
                let sample = self.Sample(
                    x_s as f32 / sample_count as f32,
                    y_s as f32 / sample_count as f32,
                    input,
                );
                output.SetPixelNorm(sample.x, sample.y, sample.color);
            }
        }
    }
}
