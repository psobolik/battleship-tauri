use crate::battleship_engine::ship::Ship;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ShipStatus {
    base: Ship,
    hits: u32,
}

impl ShipStatus {
    pub fn new(name: &str, code: char, size: usize) -> ShipStatus {
        ShipStatus {
            base: Ship::new(name.to_owned(), code, size),
            hits: 0,
        }
    }

    pub fn code(&self) -> char {
        self.base.code()
    }

    pub fn record_hit(&mut self) {
        self.hits += 1;
    }
}
