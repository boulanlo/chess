use rayon_logs::prelude::*;
use std::io::BufRead;

use crate::{Direction, PieceKind, Position};

pub struct Board {
    size: u32,
    pieces: Vec<(PieceKind, Position)>,
}

impl Board {
    /// Creates a new empty board of a given size.
    pub fn new(size: u32) -> Board {
        Board {
            size,
            pieces: Vec::new(),
        }
    }

    pub fn with_capacity(size: u32, pieces: usize) -> Board {
        Board {
            size,
            pieces: Vec::with_capacity(pieces),
        }
    }

    pub fn set_pieces(&mut self, pieces: Vec<(PieceKind, Position)>) {
        self.pieces = pieces;
    }
    pub fn clear(&mut self) {
        self.pieces.clear();
    }

    /// Adds a piece on the specified square on the board.
    ///
    /// # Panics
    /// The function panics if the square is already occupied, or if the piece to be
    /// added is a rook and one is already present on the board.
    pub fn add_piece(&mut self, piece: PieceKind, position: Position) {
        match self.get_piece(&position) {
            None => match piece {
                PieceKind::Rook => {
                    if self.is_rook_present() {
                        panic!("Trying to add a rook while it's already present on the board.")
                    } else {
                        self.pieces.push((piece, position))
                    }
                }
                _ => self.pieces.push((piece, position)),
            },
            Some(_) => panic!("Trying to add a piece on an already occupied square."),
        }
    }

    /// A helper function to add a rook on the board. Virtually similar
    /// to `board.add_piece(PieceKind::Rook, position)`.
    ///
    /// # Panics
    /// The function panics if:
    /// - The square is already occupied
    /// - The rook is already present on the board
    pub fn add_rook(&mut self, position: Position) {
        match (self.get_piece(&position), self.is_rook_present()) {
            (None, false) => self.pieces.push((PieceKind::Rook, position)),
            (None, true) => panic!("Trying to add a rook while it's already present on the board."),
            (Some(_), _) => panic!("Trying to add a piece on an already occupied square."),
        }
    }

    /// A helper function to add a bishop on the board. Virtually similar
    /// to `board.add_piece(PieceKind::Bishop, position)`.
    ///
    /// # Panics
    /// The function panics if the square is already occupied.
    pub fn add_bishop(&mut self, position: Position) {
        if self.get_piece(&position).is_some() {
            panic!("Trying to add a piece on an already occupied square.")
        } else {
            self.pieces.push((PieceKind::Bishop, position))
        }
    }

    /// A helper function to add a pawn on the board. Virtually similar
    /// to `board.add_piece(PieceKind::Pawn, position)`.
    ///
    /// # Panics
    /// The function panics if the square is already occupied.
    pub fn add_pawn(&mut self, position: Position) {
        if self.get_piece(&position).is_some() {
            panic!("Trying to add a piece on an already occupied square.")
        } else {
            self.pieces.push((PieceKind::Pawn, position))
        }
    }

    /// Returns true if the rook is present on the board, false otherwise.
    pub fn is_rook_present(&self) -> bool {
        self.pieces.iter().any(|(k, _)| match k {
            PieceKind::Rook => true,
            _ => false,
        })
    }

    /// Removes a piece on the specified square, if any.
    pub fn remove_piece(&mut self, position: &Position) {
        self.pieces.retain(|(_, p)| p != position)
    }

    /// Recreates a board from a text file. This function does not
    /// perform any checks on the I/O, and thus can panic for any
    /// I/O error that may happen during the parsing of the file.
    pub fn from_file<B: BufRead>(r: B) -> Self {
        let mut lines = r.lines();
        let board_size: u32 = lines.next().unwrap().unwrap().parse().unwrap();

        let pieces = lines
            .enumerate()
            .flat_map(|(row, l)| {
                l.unwrap()
                    .chars()
                    .enumerate()
                    .filter_map(|(col, c)| match c {
                        'p' => Some((
                            PieceKind::Pawn,
                            Position::new(row as u32, col as u32, board_size),
                        )),
                        'R' => Some((
                            PieceKind::Rook,
                            Position::new(row as u32, col as u32, board_size),
                        )),
                        'B' => Some((
                            PieceKind::Bishop,
                            Position::new(row as u32, col as u32, board_size),
                        )),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Board {
            size: board_size,
            pieces,
        }
    }

    /// Returns the kind of piece present on a certain position
    /// on the board, if any.
    pub fn get_piece(&self, position: &Position) -> Option<PieceKind> {
        self.pieces
            .iter()
            .find(|(_, p)| p == position)
            .map(|(k, _)| *k)
    }

    /// Returns the position of the rook on the board
    ///
    /// # Panic
    /// The function can panic if the board doesn't contain
    /// a rook.
    pub fn get_rook_position(&self) -> Position {
        self.pieces
            .iter()
            .find(|(k, _)| match k {
                PieceKind::Rook => true,
                _ => false,
            })
            .map(|(_, p)| *p)
            .expect("No rook present on the board.")
    }

    /// Computes the number of pawns the rook can capture in the
    /// board's current configuration.
    pub fn get_rook_captures(&self) -> usize {
        let start = self.get_rook_position();

        Direction::all() // Looking at all directions (up, down, left, right)...
            .iter()
            .map(|d| {
                match start
                    .line(*d, self.size) // ... we look at the line in that direction ...
                    .iter()
                    .filter_map(|p| self.get_piece(p))
                    .next() // .. and get the first piece on the line of sight:
                {
                    None => 0, // If there aren't any, then there is no capture
                    Some(k) => match k {
                        PieceKind::Bishop => 0, // If it's a bishop, no capture either
                        PieceKind::Pawn => 1, // If it's a pawn, we capture it
                        PieceKind::Rook => 0, // If it's another rook, no capture
                    },
                }
            })
            .sum() // ... and we sum the number of captures.
    }

    pub fn get_rook_captures_par(&self) -> usize {
        let start = self.get_rook_position();

        Direction::all()
            .into_par_iter()
            .map(|d| {
                match start
                    .line(d, self.size)
                    .iter()
                    .filter_map(|p| self.get_piece(p))
                    .next()
                {
                    None => 0,
                    Some(k) => match k {
                        PieceKind::Bishop => 0,
                        PieceKind::Pawn => 1,
                        PieceKind::Rook => unreachable!(),
                    },
                }
            })
            .sum()
    }

    /// Prints the board on the console
    pub fn print(&self) {
        (0..self.size).for_each(|row| {
            let s = (0..self.size)
                .map(|col| {
                    let kind = self.get_piece(&Position::new(row, col, self.size));

                    match kind {
                        None => '.',
                        Some(k) => match k {
                            PieceKind::Rook => 'R',
                            PieceKind::Pawn => 'p',
                            PieceKind::Bishop => 'B',
                        },
                    }
                })
                .collect::<String>();
            println!("{}", s);
        })
    }
}
