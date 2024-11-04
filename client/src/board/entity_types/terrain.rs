#[derive(Debug, Clone, Copy)]
pub struct Terrain {
    pathable: bool,
}

impl Terrain {
    pub fn new(p: bool) -> Self {
        return Self { pathable: p };
    }

    pub fn Pathable(&self) -> bool {
        return self.pathable;
    }
}
