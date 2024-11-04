extern crate entity_type;
extern crate slot_map;
use entity_type::entity_type_t;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct entity_id_t(u64);

impl entity_id_t {
    pub fn new(e_type: entity_type_t, instance: slot_map::handle_t) -> Self {
        let mut occupant_id: u64 = 0;
        occupant_id |= (instance.Handle()) as u64;
        occupant_id |= ((e_type as u64) << 48);
        return Self(occupant_id);
    }

    pub fn null() -> Self {
        return Self(0);
    }

    pub fn IsNull(&self) -> bool {
        return self.0 == 0;
    }

    pub fn Type(&self) -> entity_type_t {
        let value = (self.0 >> 48) as u16;
        return unsafe { std::mem::transmute(value) };
    }

    pub fn Handle(&self) -> slot_map::handle_t {
        return slot_map::handle_t::new((self.0 & 0x00FFFFFFFFFFFF) as u32);
    }
}
