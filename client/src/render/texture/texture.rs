extern crate image;
extern crate nalgebra;
use crate::image::GenericImageView;

pub enum Format {
    RGB,
}

pub struct Pixel {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Texture {
    pub width: u16,
    pub height: u16,
    format: Format,
    buffer: Vec<u8>,
}

impl Texture {
    pub fn FromImage(path: &str) -> Texture {
        let img = image::open(path).expect("");

        let (w, h) = img.dimensions();

        let mut output_img = Texture {
            width: w as u16,
            height: h as u16,
            buffer: Vec::with_capacity((w * h * 3) as usize),
            format: Format::RGB,
        };

        for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
            let [r, g, b] = pixel.0;
            output_img.buffer.push(r);
            output_img.buffer.push(g);
            output_img.buffer.push(b);
        }

        return output_img;
    }

    fn ToIndex(&self, x: u16, y: u16) -> usize {
        let index = ((y as usize * self.width as usize) + x as usize) * 3;
        if index >= self.buffer.len() {
            return 0;
        }
        return index;
    }

    pub fn FromDims(width: u16, height: u16) -> Texture {
        let text_size = width as usize * height as usize * 3;
        return Texture {
            width: width,
            height: height,
            buffer: vec![0; text_size],
            format: Format::RGB,
        };
    }

    pub fn SetPixelNorm(&mut self, x: f32, y: f32, pixel: Pixel) {
        let x_p = (x * self.width as f32) as u16;
        let y_p = (y * self.height as f32) as u16;
        return self.SetPixel(x_p, y_p, pixel);
    }

    pub fn SetPixel(&mut self, x: u16, y: u16, pixel: Pixel) {
        let offset = self.ToIndex(x, y);
        self.buffer[offset] = pixel.r;
        self.buffer[offset + 1] = pixel.g;
        self.buffer[offset + 2] = pixel.b;
    }

    pub fn SampleNorm(&self, x: f32, y: f32) -> Pixel {
        let x_p = (x * self.width as f32) as u16;
        let y_p = (y * self.height as f32) as u16;
        return self.Sample(x_p, y_p);
    }

    pub fn Sample(&self, x: u16, y: u16) -> Pixel {
        let offset = self.ToIndex(x, y);
        if offset >= self.buffer.len() || offset < 0 {
            return Pixel {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            };
        }

        return Pixel {
            r: self.buffer[offset],
            g: self.buffer[offset + 1],
            b: self.buffer[offset + 2],
            a: 0xff,
        };
    }
}
