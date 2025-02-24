use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct handle_t(u32);

impl handle_t {
    pub fn new(v: u32) -> Self {
        return handle_t(v);
    }

    pub fn new_null() -> Self {
        return handle_t(0);
    }

    pub fn IsNull(&self) -> bool {
        return self.0 == 0;
    }
}

/*
 * Represents a File that is loaded from disk and converted in a a struct
 * representation.
 */
pub trait IFile<T> {
    fn Load(p: &str) -> Box<T>;
    fn Size(&self) -> usize;
}

pub struct Config {
    pub max_bytes: usize,
    pub max_files: usize,
}

pub struct FileManager<T> {
    config: Config,
    handle_from_path: HashMap<String, handle_t>,
    path_from_handle: HashMap<handle_t, String>,
    open_handles: HashMap<handle_t, Box<T>>,
    used_bytes: usize,

    handle_id: Cell<u32>,
}

impl<T: IFile<T>> FileManager<T> {
    pub fn new(config: Config) -> Self {
        return Self {
            config: config,
            handle_from_path: HashMap::new(),
            path_from_handle: HashMap::new(),
            open_handles: HashMap::new(),
            handle_id: Cell::new(1),
            used_bytes: 0,
        };
    }

    /*
     * For the given path return the handle
     * TODO(oleg): make this non mutable with wrapping handles in a cell
     */
    pub fn Handle(&mut self, path: &str) -> handle_t {
        return self
            .handle_from_path
            .entry(path.to_string())
            .or_insert_with(|| {
                let new_handle = handle_t::new(self.handle_id.get() + 1);
                self.path_from_handle.insert(new_handle, path.to_string());

                self.handle_id.replace(new_handle.0);
                return new_handle.clone();
            })
            .clone();
    }

    /*
     * TODO(oleg): actually implement evicting loaded files
     */
    pub fn Load<'a>(&'a mut self, handle: handle_t) -> &'a T {
        if !self.open_handles.contains_key(&handle) {
            let path = self.path_from_handle.get(&handle).expect("");
            let allocated = T::Load(&path);
            self.used_bytes += allocated.Size();
            self.open_handles.insert(handle, allocated);
        }
        return self
            .open_handles
            .get(&handle)
            .map(|boxed_value| &**boxed_value)
            .expect("");
    }
}
