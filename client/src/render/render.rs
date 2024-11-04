extern crate board;
extern crate entity;
use entity::entity_id_t;

pub struct Application {
    pub board: board::Board,
    pub player: entity_id_t,
}

/*
 * Trait that all board renderers implement
 */
pub trait IRenderer {
    fn Render(&self, b: &Application);
}
