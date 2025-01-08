extern crate blitter;
extern crate crossterm;
extern crate file_manager;
extern crate image;
extern crate nalgebra;
extern crate profiler;
extern crate sampler;
extern crate texture;
use crate::image::GenericImageView;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use crossterm::{
    execute,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use nalgebra::Matrix4;
use nalgebra::Translation3;
use nalgebra::Vector3;
use profiler::{close_scope, open_scope};
use std::env;
use std::io;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use std::time::Instant;

struct BufferWriter {
    buffer: Vec<u8>,
    capacity: usize,
}

impl BufferWriter {
    fn new(size: usize) -> Self {
        BufferWriter {
            buffer: Vec::with_capacity(size),
            capacity: size,
        }
    }

    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len();
        if self.buffer.len() + len <= self.capacity {
            self.buffer.extend_from_slice(buf);
        } else {
            assert!(
                false,
                "BufferWriter is out of space {} < {}",
                self.buffer.len(),
                self.buffer.len() + len
            );
        }
        return Ok(len);
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let img_path = &args[1];

    let (screen_w, screen_h) = terminal::size().expect("");
    let mut ascii_buffer = BufferWriter::new(screen_w as usize * screen_h as usize * 1024);
    let textures = file_manager::FileManager::<texture::Texture>::new(file_manager::Config {
        max_bytes: 0,
        max_files: 0,
    });

    let img = texture::Texture::FromImage(img_path);
    let mut screen_buffer = texture::Texture::FromDims(screen_w, screen_h * 2);
    let mut img_sampler = sampler::Sampler {
        src_rect: Matrix4::identity(),
        dst_rect: Matrix4::identity(),
    };

    let rotation = Translation3::new(0.5, 0.5, 0.0).to_homogeneous()
        * Matrix4::from_axis_angle(&Vector3::z_axis(), 5.0_f32.to_radians())
        * Translation3::new(-0.5, -0.5, 0.0).to_homogeneous();
    let mut out = std::io::stdout();

    execute!(out, EnterAlternateScreen);
    crossterm::terminal::enable_raw_mode();

    let start = Instant::now();
    loop {
        if event::poll(std::time::Duration::from_millis(1)).expect("") {
            if let Event::Key(key_event) = event::read().expect("") {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        profiler::open_scope!("Frame");
        execute!(out, BeginSynchronizedUpdate);
        img_sampler.dst_rect = rotation * img_sampler.dst_rect;
        screen_buffer.Fill(texture::Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });
        profiler::open_scope!("GenerateOutputBuffer");
        img_sampler.Blit(&img, &mut screen_buffer);
        profiler::close_scope!();

        profiler::open_scope!("FormAscii");
        blitter::BlitImage(&screen_buffer, 0, 0, &mut ascii_buffer);
        profiler::close_scope!();

        profiler::open_scope!("PrintToScreen");
        let mut handle = out.lock();
        handle.write_all(ascii_buffer.get_buffer());
        handle.flush();
        ascii_buffer.clear();
        profiler::close_scope!();

        execute!(out, EndSynchronizedUpdate);
        profiler::close_scope!();
    }

    let duration = start.elapsed();
    crossterm::terminal::disable_raw_mode();
    execute!(out, LeaveAlternateScreen);
    println!("Time elapsed: {:?}", duration);
    println!("Screen Resolution(w,h) {} {}", screen_w, screen_h);
    profiler::print_perf!();
}
