use rand::Rng;
use std::vec;

use self::{
    direction::Direction, game_status::GameStatus, position::Position, ship::Ship,
    ship_status::ShipStatus,
};

mod direction;
mod game_status;
mod position;
mod ship;
pub mod ship_status;

static MIN_DIMENSION: usize = 10;
static MAX_DIMENSION: usize = 26;
static DEFAULT_DIMENSION: usize = 10;

static OPEN_CODE: char = '.';
static MISS_CODE: char = '*';

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleshipEngine {
    rows: usize,
    columns: usize,
    ships: Vec<Ship>,
    board: Vec<Vec<char>>,
    game_status: GameStatus,
}

impl BattleshipEngine {
    pub fn new(rows: Option<usize>, columns: Option<usize>) -> BattleshipEngine {
        let rows: usize = BattleshipEngine::clamp_dimension(rows, DEFAULT_DIMENSION);
        let columns: usize = BattleshipEngine::clamp_dimension(columns, rows);
        let ships = BattleshipEngine::ships();
        let mut battleship_engine = BattleshipEngine {
            rows,
            columns,
            ships: ships.clone(),
            board: BattleshipEngine::create_empty_board(rows, columns),
            game_status: GameStatus::new(&ships),
        };
        battleship_engine.init_board();
        battleship_engine
    }

    pub fn take_shot(&mut self, row: usize, column: usize) -> Option<&ShipStatus> {
        let position = Position::new(row, column);
        if let Some(code_at_position) = self.code_at(&position) {
            match self
                .ships
                .iter_mut()
                .find(|ship| ship.code() == code_at_position)
            {
                Some(_) => {
                    // There's an un-hit ship code at that position
                    self.set_code_at(&position, code_at_position.to_ascii_uppercase());
                    self.game_status.record_hit(code_at_position)
                }
                _ => {
                    // Either no ship at the position, or it's been hit already
                    if code_at_position == OPEN_CODE {
                        // No ship at the position
                        self.set_code_at(&position, MISS_CODE);
                    }
                    // Either way, it's a miss
                    self.game_status.record_miss();
                    None
                }
            }
        } else {
            None // Invalid position
        }
    }

    fn init_board(&mut self) {
        let mut rng = rand::rng();

        // Place the ships
        for ship in self.ships.clone() {
            loop {
                let direction = if rand::random() {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };
                let position =
                    Position::new(rng.random_range(0..self.rows), rng.random_range(0..self.columns));
                if self.ship_will_fit(&ship, &position, &direction) {
                    self.place_ship(&ship, &position, &direction);
                    break;
                }
            }
        }
    }

    fn ship_will_fit(&self, ship: &Ship, position: &Position, direction: &Direction) -> bool {
        // true
        if *direction == Direction::Horizontal {
            if position.column() + ship.size() >= self.columns {
                false // Won't fit
            } else {
                !(0..ship.size()).any(|col| {
                    !self.is_cell_free(&Position::new(position.row(), position.column() + col))
                })
            }
        } else {
            // direction == Direction::VERTICAL
            if position.row() + ship.size() >= self.rows {
                false // Won't fit
            } else {
                !(0..ship.size()).any(|row| {
                    !self.is_cell_free(&Position::new(position.row() + row, position.column()))
                })
            }
        }
    }

    fn place_ship(&mut self, ship: &Ship, position: &Position, direction: &Direction) {
        if *direction == Direction::Horizontal {
            (0..ship.size()).for_each(|col| {
                self.set_code_at(
                    &Position::new(position.row(), position.column() + col),
                    ship.code(),
                )
            });
        } else {
            // direction == Direction::VERTICAL
            (0..ship.size()).for_each(|row| {
                self.set_code_at(
                    &Position::new(position.row() + row, position.column()),
                    ship.code(),
                )
            });
        }
    }

    fn is_position_on_board(&self, position: &Position) -> bool {
        position.row() < self.rows && position.column() < self.columns
    }

    fn is_cell_free(&self, position: &Position) -> bool {
        match self.code_at(position) {
            Some(code) => code == OPEN_CODE,
            _ => false,
        }
    }

    fn code_at(&self, position: &Position) -> Option<char> {
        if self.is_position_on_board(position) {
            Some(self.board[position.row()][position.column()])
        } else {
            None
        }
    }

    fn set_code_at(&mut self, position: &Position, code: char) {
        if self.is_position_on_board(position) {
            self.board[position.row()][position.column()] = code
        }
    }
}

impl BattleshipEngine {
    fn create_empty_board(rows: usize, columns: usize) -> Vec<Vec<char>> {
        let mut board: Vec<Vec<char>> = vec![];
        (0..rows).for_each(|_| {
            let mut row: Vec<char> = vec![];
            (0..columns).for_each(|_| {
                row.push(OPEN_CODE);
            });
            board.push(row);
        });
        board
    }

    // We're trusting ourself not to pass an invalid default
    fn clamp_dimension(dimension: Option<usize>, default: usize) -> usize {
        if let Some(dimension) = dimension {
            if dimension <= MAX_DIMENSION && dimension >= MIN_DIMENSION {
                dimension
            } else {
                default
            }
        } else {
            default
        }
    }

    fn ships() -> Vec<Ship> {
        vec![
            Ship::new(String::from("Carrier"), 'c', 5),
            Ship::new(String::from("Battleship"), 'b', 4),
            Ship::new(String::from("Cruiser"), 'r', 3),
            Ship::new(String::from("Submarine"), 's', 3),
            Ship::new(String::from("Destroyer"), 'd', 2),
        ]
    }
}
