#[macro_use]
extern crate crossterm;
extern crate context;
extern crate input_manager;
extern crate renderer;

struct AppState {
    renderer: renderer::Renderer,
    input_manager: input_manager::InputManager,
}

pub fn main() {
    let mut state = AppState {
        renderer: renderer::Renderer::new(),
        input_manager: input_manager::InputManager::new(),
    };
    let mut game_context = context::Context::new(&mut state);
    game_context.Init(|app_state| {});

    loop {
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
    }

    game_context.Destroy(|app_state| {
        app_state.renderer.Destroy();
    });
}
