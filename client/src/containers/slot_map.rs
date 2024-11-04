use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct handle_t(u32);

pub trait ISlotMap {
    fn Size(&self) -> usize;
    fn Erase(&mut self);
    fn Contains(&self, handle: handle_t) -> bool;
}

impl handle_t {
    pub fn new(h: u32) -> Self {
        return handle_t(h);
    }

    pub fn CreateNull() -> handle_t {
        return handle_t(0);
    }

    pub fn IsNull(&self) -> bool {
        return self.0 == 0;
    }

    pub fn Handle(&self) -> u32 {
        return self.0;
    }
}

pub struct SlotMap<T> {
    buffer: Vec<T>,
    handle_map: HashMap<handle_t, usize>,
    last_handle: handle_t,
}

impl<T> SlotMap<T> {
    pub fn new() -> Self {
        return Self {
            buffer: Vec::new(),
            handle_map: HashMap::new(),
            last_handle: handle_t(0),
        };
    }

    pub fn Insert(&mut self, data: T) -> handle_t {
        let handle = handle_t(self.last_handle.0 + 1);
        self.last_handle = handle;
        self.buffer.push(data);
        self.handle_map.insert(handle, self.buffer.len() - 1);
        return handle;
    }

    pub fn Get(&self, h: handle_t) -> Option<&T> {
        if let Some(index) = self.handle_map.get(&h) {
            return Some(&self.buffer[index.clone()]);
        }
        return None;
    }

    pub fn GetMut(&mut self, h: handle_t) -> Option<&mut T> {
        if let Some(index) = self.handle_map.get(&h) {
            return Some(&mut self.buffer[index.clone()]);
        }
        return None;
    }
}

impl<T> ISlotMap for SlotMap<T> {
    fn Size(&self) -> usize {
        return self.buffer.len();
    }

    fn Erase(&mut self) {}

    fn Contains(&self, handle: handle_t) -> bool {
        return true;
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for SlotMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for item in &self.buffer {
            result.push_str(&format!(",{:?}", item));
        }
        return write!(f, "{}", result);
    }
}
