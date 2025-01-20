#[macro_use]
extern crate crossterm;
extern crate context;
extern crate input_manager;
extern crate renderer;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
struct AppState {
    renderer: renderer::Renderer,
    input_manager: input_manager::InputManager,
}

pub fn main() {
    let target_frame_time_ms = 32;

    let mut state = AppState {
        renderer: renderer::Renderer::new(),
        input_manager: input_manager::InputManager::new(),
    };
    let mut game_context = context::Context::new(&mut state);
    game_context.Init(|app_state| {});

    loop {
        let frame_start = SystemTime::now();
        game_context.TickRender(|app_state, texture_manager, sprite_list| {
            app_state
                .renderer
                .Render(sprite_list.iter(), texture_manager);
        });

        let keep_running = game_context.TickInput(|app_state| {
            return vec![app_state.input_manager.PollInput()];
        });

        if (!keep_running) {
            break;
        }
        let delta = SystemTime::now()
            .duration_since(frame_start)
            .expect("")
            .as_millis() as u64;
        if (delta > target_frame_time_ms) {
            continue;
        }
        thread::sleep(Duration::from_millis(delta));
    }

    game_context.Destroy(|app_state| {
        app_state.renderer.Destroy();
    });
}
