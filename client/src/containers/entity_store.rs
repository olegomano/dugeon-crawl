extern crate entity;
extern crate entity_type;
extern crate player;
extern crate slot_map;
extern crate terrain;
use entity::entity_id_t;
use entity_type::entity_type_t;
use std::collections::HashMap;
use std::fmt;
use std::pin::Pin;

/*
 * The memmory backing where all the entities are stored
 * Cells contains handles into the EntityStore to look up the actual date in them
 */
struct EntityStoreImpl {
    terrain: slot_map::SlotMap<terrain::Terrain>,
    player: slot_map::SlotMap<player::Player>,
}

/**
 * TODO(oleg): figure out a way to automate generation of this
 * we really just want to provide a list of types + type_id and then have a callback
 * and members automatically generated for them
 */
impl EntityStoreImpl {
    pub fn new() -> Self {
        return Self {
            terrain: slot_map::SlotMap::new(),
            player: slot_map::SlotMap::new(),
        };
    }

    pub fn DebugString(&self) -> String {
        let mut result = String::new();
        return result;
    }

    fn Init<F>(self: Pin<&mut Self>, mut f: F)
    where
        F: FnMut(entity_type::entity_type_t, *const ()),
    {
        {
            let ptr: *const slot_map::SlotMap<terrain::Terrain> = &self.terrain;
            f(entity_type_t::TERRAIN, ptr as *const ());
        }

        {
            let ptr: *const slot_map::SlotMap<player::Player> = &self.player;
            f(entity_type_t::PLAYER, ptr as *const ());
        }
    }
}

pub struct EntityStore {
    store: Pin<Box<EntityStoreImpl>>,
    entity_lut: HashMap<entity_type::entity_type_t, *const ()>,
}

impl EntityStore {
    pub fn new() -> Self {
        let mut store_impl = Box::pin(EntityStoreImpl::new());
        let mut lut = HashMap::<entity_type_t, *const ()>::new();

        store_impl.as_mut().Init(|id, ptr| {
            lut.insert(id, ptr);
        });

        return Self {
            store: store_impl,
            entity_lut: lut,
        };
    }

    pub fn DebugString(&self) -> String {
        let mut result = String::new();

        return result;
    }

    pub fn GetEntity<T>(&self, e: entity_id_t) -> Option<&T> {
        if let Some(slot_map) = self.GetEntities::<T>(e.Type()) {
            return slot_map.Get(e.Handle());
        }
        return None;
    }

    pub fn GetEntityMut<T>(&mut self, e: entity_id_t) -> Option<&mut T> {
        if let Some(slot_map) = self.GetEntitiesMut::<T>(e.Type()) {
            return slot_map.GetMut(e.Handle());
        }
        return None;
    }

    pub fn GetEntities<T>(&self, t: entity_type_t) -> Option<&slot_map::SlotMap<T>> {
        if let Some(ptr) = self.entity_lut.get(&t) {
            let slot_map = unsafe { &*(*ptr as *const slot_map::SlotMap<T>) };
            return Some(&slot_map);
        }
        return None;
    }

    pub fn GetEntitiesMut<T>(&mut self, t: entity_type_t) -> Option<&mut slot_map::SlotMap<T>> {
        if let Some(ptr) = self.entity_lut.get(&t) {
            let slot_map = unsafe { &mut *(*ptr as *mut slot_map::SlotMap<T>) };
            return Some(slot_map);
        }
        return None;
    }

    pub fn CreateEntity<T>(&mut self, t: entity_type_t, data: T) -> entity_id_t {
        println!("Creating new entity of type {:?}", t);
        if let Some(slot_map) = self.GetEntitiesMut::<T>(t) {
            return entity_id_t::new(t, slot_map.Insert(data));
        }
        return entity_id_t::null();
    }
}

impl std::fmt::Display for EntityStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}\n{}", self.store.player, self.store.terrain);
    }
}
