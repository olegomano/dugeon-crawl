extern crate crossterm;
extern crate sampler;
extern crate texture;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{style, Color, Print, PrintStyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::convert::From;
use std::fmt::Write as FmtWrite;
use std::io::stdout;
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::{Duration, Instant};

fn ToColor(t: texture::Pixel) -> Color {
    return Color::Rgb {
        r: t.r,
        g: t.g,
        b: t.b,
    };
}

fn MakeColor(r: u8, g: u8, b: u8) -> Color {
    return Color::Rgb { r: r, g: g, b: b };
}

/*
 * Blits the image into the out buffer at the x,y position
 * Renders horizontally in columns of two
 */
pub fn BlitImage<T: Write>(image: &texture::Texture, x: u16, y: u16, out: &mut T) {
    let upper_half = "▀";
    let lower_half = "▄";
    let full_pixel = "█";
    let emtpy_pixel = " ";

    //we draw two rows at a time because of how the terminal symbols work
    for p_y in 0..image.height / 2 {
        let y_offset = y + p_y as u16;
        for p_x in 0..image.width {
            let top_color = image.Sample(p_x, p_y * 2);
            let bottom_color = image.Sample(p_x, p_y * 2 + 1);

            let styled = style(upper_half)
                .with(ToColor(top_color))
                .on(ToColor(bottom_color));
            execute!(out, cursor::MoveTo(p_x, p_y), PrintStyledContent(styled));
        }
    }
}
