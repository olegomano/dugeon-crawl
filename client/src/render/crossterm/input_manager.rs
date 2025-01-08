extern crate input;
extern crate nalgebra;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::time::Instant;

pub struct InputManager {}

impl InputManager {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn PollInput(&self) -> input::Action {
        if event::poll(std::time::Duration::from_millis(1)).expect("") {
            if let Event::Key(key_event) = event::read().expect("") {
                if key_event.code == KeyCode::Char('w') {
                    return input::Action::Move(nalgebra::Vector4::<f32>::new(
                        0.0, -0.01, 0.0, 0.0,
                    ));
                }
                if key_event.code == KeyCode::Char('s') {
                    return input::Action::Move(nalgebra::Vector4::<f32>::new(0.0, 0.01, 0.0, 0.0));
                }
                if key_event.code == KeyCode::Char('a') {
                    return input::Action::Move(nalgebra::Vector4::<f32>::new(0.01, 0.0, 0.0, 0.0));
                }
                if key_event.code == KeyCode::Char('d') {
                    return input::Action::Move(nalgebra::Vector4::<f32>::new(
                        -0.01, 0.0, 0.0, 0.0,
                    ));
                }
                if key_event.code == KeyCode::Char('q') {
                    return input::Action::Quit();
                }
            }
        }
        return input::Action::None();
    }
}
