use crate::battleship_engine::ship::Ship;
use crate::battleship_engine::ship_status::ShipStatus;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GameStatus {
    shots: u32,
    hits: u32,
    misses: u32,
    ship_statuses: Vec<ShipStatus>,
}

impl GameStatus {
    pub fn new(ships: &[Ship]) -> GameStatus {
        let mut ship_statuses: Vec<ShipStatus> = vec![];
        for ship in ships {
            ship_statuses.push(ShipStatus::new(ship.name(), ship.code(), ship.size()))
        }
        GameStatus {
            shots: 0,
            hits: 0,
            misses: 0,
            ship_statuses,
        }
    }

    fn record_shot(&mut self) {
        self.shots += 1;
    }

    pub fn record_miss(&mut self) {
        self.record_shot();
        self.misses += 1;
    }

    pub fn record_hit(&mut self, ship_code: char) -> Option<&ShipStatus> {
        self.record_shot();
        if let Some(hit_ship) = self
            .ship_statuses
            .iter_mut()
            .find(|ship_status| ship_status.code() == ship_code)
        {
            hit_ship.record_hit();
            self.hits += 1;
            Some(hit_ship)
        } else {
            None
        }
    }

    // pub fn any_afloat(&self) -> bool {
    //     self.ship_statuses
    //         .iter()
    //         .any(|ship_status| ship_status.is_afloat())
    // }
}
