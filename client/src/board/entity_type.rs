#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(u16)]
pub enum entity_type_t {
    TERRAIN = 1,
    PLAYER = 2,
}
