extern crate entity;
extern crate entity_type;
use entity_type::entity_type_t;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct cell_id_t(u64);

impl cell_id_t {
    pub fn new(x: u32, y: u32) -> Self {
        let x_wide = x as u64;
        let y_wide = y as u64;
        return Self((x_wide << 32) | y_wide);
    }

    pub fn null() -> Self {
        return Self(0xFFFFFFFFFFFFFFFF);
    }

    pub fn IsNull(&self) -> bool {
        return self.0 == Self::null().0;
    }

    pub fn X(&self) -> u32 {
        return ((self.0 & 0xFFFFFFFF00000000) >> 32) as u32;
    }

    pub fn Y(&self) -> u32 {
        return (self.0 & 0xFFFFFFFF) as u32;
    }
}

/*
 * Represents a single unit on the board
 */
pub struct Cell {
    id: cell_id_t,
    occupants_by_type: HashMap<entity_type_t, HashSet<entity::entity_id_t>>,
}

/*
 *
 *
 */
impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        return Self {
            id: cell_id_t::new(x, y),
            occupants_by_type: HashMap::new(),
        };
    }

    pub fn DebugString(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!(
            "({},{}): {} ",
            self.id.X(),
            self.id.Y(),
            self.occupants_by_type.len()
        ));
        for (t, id) in &self.occupants_by_type {
            result.push_str(&format!("{:?} -> {:?} ", t, id.len()));
            for i in id {
                result.push_str(&format!("{:?} {:?},", i.Type(), i.Handle()));
            }
        }
        return result;
    }

    pub fn Id(&self) -> cell_id_t {
        return self.id;
    }

    pub fn ContainsEntityType(&self, t: entity_type_t) -> bool {
        if let Some(entities) = self.occupants_by_type.get(&t) {
            return entities.len() > 0;
        }
        return false;
    }

    pub fn RemoveEntity(&mut self, entity: entity::entity_id_t) {
        if let Some(entities) = self.occupants_by_type.get_mut(&entity.Type()) {
            entities.remove(&entity);
        }
    }

    pub fn AddEntity(&mut self, entity: entity::entity_id_t) {
        self.occupants_by_type
            .entry(entity.Type())
            .or_insert_with(HashSet::new)
            .insert(entity);
    }

    pub fn EntitiesOfType(&self, t: entity_type_t) -> Vec<entity::entity_id_t> {
        let mut result: Vec<entity::entity_id_t> = Vec::new();
        if let Some(entities) = self.occupants_by_type.get(&t) {
            for e in entities {
                result.push(*e);
            }
        }
        return result;
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.DebugString());
    }
}
