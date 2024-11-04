#[macro_use]
extern crate log;
extern crate cell;
extern crate crossterm;
extern crate entity;
extern crate entity_type;
extern crate player;
extern crate render;
extern crate terminal_renderer;
use crate::crossterm::style::Stylize;
use crossterm::{
    cursor,
    style::{style, Color, PrintStyledContent},
    terminal, QueueableCommand,
};
use std::error::Error;
use std::io::stdout;
use std::io::Write;
use std::time::{Duration, Instant};

pub fn main() {
    println!("Hello World");
    let mut board = board::Board::GenerateRandom();

    let player = board.NewEntity(
        cell::cell_id_t::new(0, 0),
        entity_type::entity_type_t::PLAYER,
        player::Player::new(),
    );

    let mut app = render::Application {
        board: board,
        player: player,
    };
    println!("{}", app.board.DebugString());
    println!("{}", app.board.EntityStore());

    let renderer = terminal_renderer::TerminalRenderer::new();
    terminal_renderer::TerminalApp::Run(&mut app, &renderer);
}
