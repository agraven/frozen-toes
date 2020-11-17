const BOARD_SIZE: usize = 3;

/// A value of a field on the board
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Field {
    Cross,
    Circle,
    Blank,
}

impl Default for Field {
    fn default() -> Self {
        Field::Blank
    }
}

/// A tic tac toe board
#[derive(Debug, Default)]
pub struct Board([[Field; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    /// Constructs a blank gameboard
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the given row
    pub fn row(&self, row: usize) -> [Field; BOARD_SIZE] {
        self.0[row]
    }

    /// Gets the given column
    pub fn column(&self, col: usize) -> [Field; BOARD_SIZE] {
        [self.0[0][col], self.0[1][col], self.0[2][col]]
    }

    /// Determines if the board has a winner
    pub fn winner(&self) -> Field {
        let board = &self.0;

        for &winner in &[Field::Circle, Field::Cross] {
            // Check rows
            for row in board.iter() {
                if row.iter().all(|&field| field == winner) {
                    return winner;
                }
            }
            // Check column
            for column in (0..BOARD_SIZE).map(|i| self.column(i)) {
                if column.iter().all(|&field| field == winner) {
                    return winner;
                }
            }
            // Check diagonally NW to SE
            if (0..BOARD_SIZE)
                .map(|i| self.0[i][i])
                .all(|field| field == winner)
            {
                return winner;
            }
            // Check diagonally NE to SW
            if (0..BOARD_SIZE)
                .map(|i| self.0[i][BOARD_SIZE - 1 - i])
                .all(|field| field == winner)
            {
                return winner;
            }
        }
        Field::Blank
    }
}
