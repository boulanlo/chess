use rand::{prelude::IteratorRandom, Rng};

/// The four cardinal directions. North and south mean going
/// up and down the rows (i.e. the numbers), while east and
/// west mean going up and down the columns (i.e. the letters)
/// on the chessboard.
#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns the vector used to calculate a new position based on
    /// the direction
    ///
    /// # Examples
    /// ```
    /// assert_eq!(Direction::North.get_vector(), (0, -1))
    /// ```
    pub fn get_vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    /// Returns all cardinal directions as a vector.
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

/// A position on the chessboard, identified by the row and column numbers.
/// For the sake of simplicity, we consider the columns as numbers, as opposed
/// to letters in traditional chess notation.
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Position {
    row: u32,
    col: u32,
}

impl Position {
    /// Returns a Position represented by the row and column number
    /// passed as parameters.
    ///
    /// # Example
    /// ```
    /// let position = Position::new(3, 4);
    ///
    /// assert_eq!(position.row, 3);
    /// ```
    ///
    /// # Panics
    /// The function panics if the row or the column is greater or equal to `board_size`.
    pub fn new(row: u32, col: u32, board_size: u32) -> Self {
        assert!(row < board_size);
        assert!(col < board_size);

        Position { row, col }
    }

    /// Returns a list of positions representing a line from the initial position
    /// to the edge of the board, in the direction specified as parameters.
    ///
    /// # Examples
    /// ```
    /// let position = Position::new(2, 3);
    ///
    /// assert_eq!(
    ///     position.line(Direction::North),
    ///     vec![
    ///         Position::new(2,2),
    ///         Position::new(2,1),
    ///         Position::new(2, 0)
    ///     ]
    /// );
    /// ```
    pub fn line(&self, direction: Direction, board_size: u32) -> Vec<Position> {
        match direction {
            Direction::West => (0..self.col)
                .map(|i| Position::new(self.row, i, board_size))
                .rev()
                .collect(),
            Direction::East => (board_size.min(self.col + 1)..board_size)
                .map(|i| Position::new(self.row, i, board_size))
                .collect(),
            Direction::South => (board_size.min(self.row + 1)..board_size)
                .map(|i| Position::new(i, self.col, board_size))
                .collect(),
            Direction::North => (0..self.row)
                .map(|i| Position::new(i, self.col, board_size))
                .rev()
                .collect(),
        }
    }

    /// Returns a random position
    pub fn random<R: Rng>(random: &mut R, max: u32) -> Position {
        Position {
            row: random.gen_range(0, max),
            col: random.gen_range(0, max),
        }
    }

    /// Generates random unique positions
    pub fn generate_unique_positions<R: Rng>(
        random: &mut R,
        count: u32,
        max: u32,
    ) -> Vec<Position> {
        (0..max)
            .flat_map(|row| (0..max).map(move |col| Position::new(row, col, max)))
            .choose_multiple(random, count as usize)
    }
}
