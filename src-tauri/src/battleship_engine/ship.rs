#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Ship {
    name: String,
    code: char,
    size: usize,
}

impl Ship {
    pub fn new(name: String, code: char, size: usize) -> Ship {
        Ship { name, code, size }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn code(&self) -> char {
        self.code
    }
    pub fn size(&self) -> usize {
        self.size
    }
}
