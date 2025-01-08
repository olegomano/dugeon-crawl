extern crate crossterm;
extern crate file_manager;
extern crate sampler;
extern crate sprite;
extern crate texture;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use crossterm::{
    execute,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use nalgebra::matrix;
use nalgebra::Matrix4;
use nalgebra::Vector4;
use std::io;
use std::io::{stdout, Write};

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

pub struct Renderer {
    screen_buffer: texture::Texture,
    ascii_buffer: BufferWriter,
}

impl Renderer {
    pub fn new() -> Self {
        let (screen_w, screen_h) = terminal::size().expect("");
        let ascii_buffer = BufferWriter::new(screen_w as usize * screen_h as usize * 1024);
        let screen_buffer = texture::Texture::FromDims(screen_w, screen_h * 2);

        let mut out = std::io::stdout();
        execute!(out, EnterAlternateScreen);
        crossterm::terminal::enable_raw_mode();

        return Self {
            screen_buffer: screen_buffer,
            ascii_buffer: ascii_buffer,
        };
    }

    pub fn Render<'b, I>(
        &mut self,
        iter: I,
        texture_manager: &mut file_manager::FileManager<texture::Texture>,
    ) where
        I: Iterator<Item = &'b sprite::Sprite>,
    {
        self.screen_buffer.Fill(texture::Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        for sprite in iter {
            let sampler = sampler::Sampler {
                src_rect: Matrix4::identity(),
                dst_rect: *sprite.Transform(),
            };
            sampler.Blit(
                texture_manager.Load(sprite.Texture()),
                &mut self.screen_buffer,
            );
        }
        blitter::BlitImage(&self.screen_buffer, 0, 0, &mut self.ascii_buffer);

        let mut out = std::io::stdout();
        let mut handle = out.lock();
        handle.write_all(self.ascii_buffer.get_buffer());
        handle.flush();
        self.ascii_buffer.clear();
    }

    pub fn Destroy(&mut self) {
        let mut out = std::io::stdout();
        crossterm::terminal::disable_raw_mode();
        execute!(out, LeaveAlternateScreen);
    }
}
