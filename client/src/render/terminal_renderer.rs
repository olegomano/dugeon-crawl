extern crate board;
extern crate cell;
extern crate crossterm;
extern crate entity;
extern crate entity_type;
extern crate player;
extern crate render;
use crate::crossterm::style::Stylize;
use crossterm::{
    cursor,
    style::{style, Color, PrintStyledContent},
    terminal, QueueableCommand,
};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
};
use entity_type::entity_type_t;
use std::error::Error;
use std::io::stdout;
use std::io::Write;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

pub struct TerminalApp {}

impl TerminalApp {
    pub fn Run(board: &mut render::Application, renderer: &dyn render::IRenderer) {
        let frame_time = Duration::from_secs_f64(1.0 / 10.0);
        stdout().execute(crossterm::terminal::EnterAlternateScreen);
        crossterm::terminal::enable_raw_mode();
        loop {
            match (crossterm::event::poll(frame_time)) {
                Ok(result) => {
                    if let Event::Key(key_event) = event::read().expect("Failed to read keycode") {
                        match key_event.code {
                            KeyCode::Char('w') => {
                                let player_cell = board.board.GetEntityCell(board.player);
                                let next_cell =
                                    cell::cell_id_t::new(player_cell.X(), player_cell.Y() + 1);
                                board.board.MoveEntity(board.player, next_cell);
                            }
                            KeyCode::Char('s') => {
                                let player_cell = board.board.GetEntityCell(board.player);
                                let next_cell =
                                    cell::cell_id_t::new(player_cell.X(), player_cell.Y() - 1);
                                board.board.MoveEntity(board.player, next_cell);
                            }
                            KeyCode::Char('q') => {
                                crossterm::terminal::disable_raw_mode();
                                stdout().execute(crossterm::terminal::LeaveAlternateScreen);
                                process::exit(0);
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => {}
            }
            renderer.Render(board);
        }
    }
}

pub struct TerminalRenderer {}

impl TerminalRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    fn PrintChar(out: &mut std::io::Stdout, c: char, fg: Color, bg: Color) {
        let styled = style(c).with(fg).on(bg);
        out.queue(PrintStyledContent(styled));
    }

    fn PrintBorder(out: &mut std::io::Stdout) {
        Self::PrintChar(
            out,
            ' ',
            Color::Rgb {
                r: 128,
                g: 128,
                b: 128,
            },
            Color::Rgb {
                r: 128,
                g: 128,
                b: 128,
            },
        );
    }

    fn PrintNonPathable(out: &mut std::io::Stdout) {
        Self::PrintChar(
            out,
            ' ',
            Color::Rgb {
                r: 200,
                g: 200,
                b: 200,
            },
            Color::Rgb {
                r: 200,
                g: 200,
                b: 200,
            },
        );
    }

    fn PrintPlayer(out: &mut std::io::Stdout) {
        Self::PrintChar(
            out,
            '+',
            Color::Rgb {
                r: 200,
                g: 200,
                b: 200,
            },
            Color::Rgb { r: 255, g: 0, b: 0 },
        );
    }
}

impl render::IRenderer for TerminalRenderer {
    fn Render(&self, app: &render::Application) {
        let mut stdout = stdout();
        let (width, height) = terminal::size().expect("Failed to get terminal size");
        let width_i32 = width as i32;
        let height_i32 = height as i32;

        //self.Clear(&mut stdout);
        let player_cell = app.board.GetEntityCell(app.player);

        let cell_x_offset = player_cell.X() as i32 - width_i32 / 2;
        let cell_y_offset = player_cell.Y() as i32 - height_i32 / 2;

        stdout.queue(cursor::MoveTo(0, 0));
        for y in 0..height_i32 {
            for x in 0..width_i32 {
                let cell_x = x + cell_x_offset;
                let cell_y = y + cell_y_offset;
                if cell_x < 0
                    || cell_x >= app.board.Width()
                    || cell_y < 0
                    || cell_y >= app.board.Height()
                {
                    Self::PrintBorder(&mut stdout);
                } else {
                    let cell_id = cell::cell_id_t::new(cell_x as u32, cell_y as u32);

                    let cell = app
                        .board
                        .GetCell(cell_id)
                        .expect(&format!("Failed to get cell {} {}", x, y));

                    let terrain_id = cell.EntitiesOfType(entity_type_t::TERRAIN);
                    let player_id = cell.EntitiesOfType(entity_type_t::PLAYER);

                    if terrain_id.len() == 0 {
                        Self::PrintBorder(&mut stdout);
                    } else if player_id.len() == 0 {
                        let terrain = app
                            .board
                            .GetEntity::<terrain::Terrain>(terrain_id[0])
                            .expect("");

                        if terrain.Pathable() {
                            Self::PrintChar(
                                &mut stdout,
                                ' ',
                                Color::Rgb { r: 0, g: 0, b: 0 },
                                Color::Rgb { r: 0, g: 0, b: 0 },
                            );
                        } else {
                            Self::PrintNonPathable(&mut stdout);
                        }
                    } else {
                        /**
                        let player = app
                            .board
                            .GetEntity::<player::Player>(player_id[0])
                            .expect("");
                        **/
                        Self::PrintPlayer(&mut stdout);
                    }
                }
            }
        }
        stdout.flush();
    }
}
