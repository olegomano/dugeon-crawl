extern crate crossterm;
extern crate sampler;
extern crate texture;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{style, Color, PrintStyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::convert::From;
use std::io::stdout;
use std::io::Write;
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
pub fn BlitImage(image: &texture::Texture, x: u16, y: u16, out: &mut std::io::Stdout) {
    let upper_half = "▀";
    let lower_half = "▄";
    let full_pixel = "█";
    let emtpy_pixel = " ";

    //we draw two rows at a time because of how the terminal symbols work
    for p_y in 0..image.height / 2 {
        let y_offset = y + p_y as u16;
        out.queue(cursor::MoveTo(x, y_offset));
        for p_x in 0..image.width {
            let top_color = image.Sample(p_x, p_y * 2);
            let bottom_color = image.Sample(p_x, p_y * 2 + 1);

            let styled = style(upper_half)
                .with(ToColor(top_color))
                .on(ToColor(bottom_color));
            out.queue(PrintStyledContent(styled));
        }
    }
}

/*
 * Clears the terminal
 */
pub fn Clear(out: &mut std::io::Stdout) {
    let (w, h) = terminal::size().expect("");
    for x in 0..w {
        for y in 0..h {
            out.queue(cursor::MoveTo(x, y));
            let styled = style(" ").with(MakeColor(0, 0, 0)).on(MakeColor(0, 0, 0));
            out.queue(PrintStyledContent(styled));
        }
    }
}
