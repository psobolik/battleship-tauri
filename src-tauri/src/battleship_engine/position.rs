#[derive(PartialEq, Eq)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        Position { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }
    pub fn column(&self) -> usize {
        self.column
    }
}
