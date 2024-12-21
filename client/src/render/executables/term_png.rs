extern crate blit_renderer;
extern crate crossterm;
extern crate image;
extern crate nalgebra;
extern crate sampler;
extern crate texture;
use crate::image::GenericImageView;
use crossterm::terminal;
use nalgebra::Matrix4;
use std::env;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let img_path = &args[1];

    let (screen_w, screen_h) = terminal::size().expect("");
    let img = texture::Texture::FromImage(img_path);
    let mut screen_buffer = texture::Texture::FromDims(screen_w, screen_h * 2);
    let mut img_sampler = sampler::Sampler {
        src_rect: Matrix4::identity(),
        dst_rect: Matrix4::identity(),
    };

    let rotation_matrix =
        Matrix4::from_axis_angle(&nalgebra::Vector3::z_axis(), 15.0_f32.to_radians());

    crossterm::terminal::enable_raw_mode();
    let start = Instant::now();
    let mut stdout = std::io::stdout();

    loop {
        img_sampler.dst_rect = rotation_matrix * img_sampler.dst_rect;
        img_sampler.Blit(&img, &mut screen_buffer);
        blit_renderer::BlitImage(&screen_buffer, 0, 0, &mut stdout);
    }

    let duration = start.elapsed();
    crossterm::terminal::disable_raw_mode();
    println!("Time elapsed: {:?}", duration);
}
