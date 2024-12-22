extern crate blit_renderer;
extern crate crossterm;
extern crate image;
extern crate nalgebra;
extern crate sampler;
extern crate texture;
use crate::image::GenericImageView;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use nalgebra::Matrix4;
use nalgebra::Translation3;
use nalgebra::Vector3;
use std::env;
use std::io::{stdout, Write};
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

    let rotation = Translation3::new(0.5, 0.5, 0.0).to_homogeneous()
        * Matrix4::from_axis_angle(&Vector3::z_axis(), 1.0_f32.to_radians())
        * Translation3::new(-0.5, -0.5, 0.0).to_homogeneous();
    let mut out = std::io::stdout();

    crossterm::terminal::enable_raw_mode();
    let start = Instant::now();
    execute!(out, EnterAlternateScreen);

    loop {
        if event::poll(std::time::Duration::from_millis(10)).expect("") {
            if let Event::Key(key_event) = event::read().expect("") {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        img_sampler.dst_rect = rotation * img_sampler.dst_rect;
        screen_buffer.Fill(texture::Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });
        img_sampler.Blit(&img, &mut screen_buffer);
        blit_renderer::BlitImage(&screen_buffer, 0, 0, &mut out);
    }

    let duration = start.elapsed();
    crossterm::terminal::disable_raw_mode();
    execute!(out, LeaveAlternateScreen);
    println!("Time elapsed: {:?}", duration);
}
