use std::collections::HashMap;
use std::collections::HashSet;
extern crate cell;
extern crate entity;
extern crate entity_store;
extern crate entity_type;
extern crate terrain;
use entity::entity_id_t;

type board_id_t = i64;

/*
 * Represents the board state of the dungeon crawl
 * A Board is a collection of cells
 */
pub struct Board {
    id: board_id_t,
    width: i32,
    height: i32,
    loaded_cell_map: HashMap<cell::cell_id_t, cell::Cell>,
    entity_cell_map: HashMap<entity::entity_id_t, cell::cell_id_t>,
    entity_store: entity_store::EntityStore,
}

impl Board {
    pub fn new(w: i32, h: i32) -> Self {
        println!("Creating new board of size {},{}", w, h);
        let mut result = Self {
            id: 0,
            width: w,
            height: h,
            loaded_cell_map: HashMap::new(),
            entity_cell_map: HashMap::new(),
            entity_store: entity_store::EntityStore::new(),
        };

        for x in 0..w {
            for y in 0..h {
                let cell = cell::Cell::new(x as u32, y as u32);
                result.loaded_cell_map.insert(cell.Id(), cell);
            }
        }
        return result;
    }

    pub fn EntityStore(&self) -> &entity_store::EntityStore {
        return &self.entity_store;
    }

    pub fn DebugString(&self) -> String {
        let mut result = String::new();
        for (cell_id, cell) in &self.loaded_cell_map {
            result.push_str(&cell.DebugString());
            result.push_str("\n");
        }
        return result;
    }

    pub fn Width(&self) -> i32 {
        return self.width;
    }

    pub fn Height(&self) -> i32 {
        return self.height;
    }

    /*
     * Returns the cell that the entity is currently located in
     */
    pub fn GetEntityCell(&self, entity: entity::entity_id_t) -> cell::cell_id_t {
        if let Some(cell) = self.entity_cell_map.get(&entity) {
            return cell.clone();
        }
        return cell::cell_id_t::null();
    }

    pub fn GetEntity<T>(&self, entity: entity_id_t) -> Option<&T> {
        return self.entity_store.GetEntity::<T>(entity);
    }

    pub fn GetCell(&self, cell_id: cell::cell_id_t) -> Option<&cell::Cell> {
        if let Some(cell) = self.loaded_cell_map.get(&cell_id) {
            return Some(cell);
        }
        return None;
    }

    /*
     * Creates a new entity and places it into the target cell
     */
    pub fn NewEntity<T>(
        &mut self,
        c: cell::cell_id_t,
        t: entity_type::entity_type_t,
        data: T,
    ) -> entity::entity_id_t {
        println!("Creating new entity {:?} in cell {},{}", t, c.X(), c.Y());

        if let Some(cell_instance) = self.loaded_cell_map.get_mut(&c) {
            let entity = self.entity_store.CreateEntity(t, data);
            cell_instance.AddEntity(entity);
            self.entity_cell_map.insert(entity, c);
            return entity;
        }
        return entity::entity_id_t::null();
    }

    /**
     * Move entity.
     *
     * Moves the specified entity from the source to the target cell
     */
    pub fn MoveEntity(&mut self, e: entity::entity_id_t, target_cell_id: cell::cell_id_t) {
        let old_cell_id = self.GetEntityCell(e);

        //should be pointer to Cell
        let old_ptr = self
            .loaded_cell_map
            .get_mut(&old_cell_id)
            .map(|x| x as *mut cell::Cell)
            .expect("old_cell_id is missing or null");

        let new_ptr = self
            .loaded_cell_map
            .get_mut(&target_cell_id)
            .map(|x| x as *mut cell::Cell)
            .expect("target_cell_id is missing or null");

        self.entity_cell_map.insert(e, target_cell_id);
        unsafe {
            //rust can't have two mut references to members of a cell :(
            (*old_ptr).RemoveEntity(e);
            (*new_ptr).AddEntity(e);
        }
    }
}

impl Board {
    pub fn GenerateRandom() -> Self {
        let width = 32;
        let height = 32;

        let mut result = Board::new(width, height);

        for x in 0..width {
            for y in 0..height {
                result.NewEntity(
                    cell::cell_id_t::new(x as u32, y as u32),
                    entity_type::entity_type_t::TERRAIN,
                    terrain::Terrain::new(x == y),
                );
            }
        }
        return result;
    }
}
