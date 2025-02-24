#[macro_use]
extern crate context;
extern crate crossterm_platform_impl;
extern crate test_scene;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn main() {
    let target_frame_time_ms = 32;
    let crossterm_platform = crossterm_platform_impl::Renderer::new();
    let test_scene = test_scene::TestScene::new();

    let mut game_context = context::Context::new(crossterm_platform, test_scene);
    game_context.Init();
    loop {
        let frame_start = SystemTime::now();
        if (!game_context.Tick()) {
            break;
        }
    }
    game_context.Destroy();
}
