extern crate char_controller;
extern crate file_manager;
extern crate game_object;
extern crate input;
extern crate nalgebra;
extern crate slot_map;
extern crate sprite;
extern crate texture;

pub struct TestScene {
    player: char_controller::CharController,
}

impl TestScene {
    pub fn new() -> Self {
        return Self {
            player: char_controller::CharController::new_null(),
        };
    }
}

impl game_object::IScene for TestScene {
    fn OnInit(&mut self, context: &mut game_object::GameContext) {
        self.player = char_controller::CharController::new_sprite(&char_controller::SheetConfig {
            cell_count_w: 2,
            cell_count_h: 2,
            image_handle: context
                .texture_manager
                .Handle("/home/oleg/Documents/Dev/DungeonCrawl/asset/mage.png"),
        });
        self.player.Init(context);
    }

    fn OnTick(&mut self, context: &mut game_object::GameContext) {}

    fn OnInput(&mut self, context: &mut game_object::GameContext, input: &input::Action) {
        self.player.HandleAction(input, context);
    }
}
