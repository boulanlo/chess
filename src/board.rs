use rayon_logs::prelude::*;
use std::collections::HashSet;
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
    /// The function panics if the square is already occupied.
    pub fn add_piece(&mut self, piece: PieceKind, position: Position) {
        match self.get_piece(&position) {
            None => self.pieces.push((piece, position)),
            Some(_) => panic!("Trying to add a piece on an already occupied square."),
        }
    }

    /// A helper function to add a rook on the board. Virtually similar
    /// to `board.add_piece(PieceKind::Rook, position)`.
    ///
    /// # Panics
    /// The function panics if:
    /// - The square is already occupied,
    pub fn add_rook(&mut self, position: Position) {
        if self.get_piece(&position).is_some() {
            panic!("Trying to add a piece on an already occupied square.");
        } else {
            self.pieces.push((PieceKind::Rook, position));
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

    pub fn get_rooks_positions(&self) -> Vec<Position> {
        self.pieces
            .iter()
            .filter_map(|(k, p)| match k {
                PieceKind::Rook => Some(*p),
                _ => None,
            })
            .collect()
    }

    /// Computes the number of pawns the rook can capture in the
    /// board's current configuration.
    pub fn get_rook_captures(&self) -> usize {
        let start = self.get_rook_position();

        // Looking at all directions (up, down, left, right):
        Direction::all()
            .iter()
            .map(|d| {
                match start
                    // we look at the line in that direction
                    .line(*d, self.size)
                    .iter()
                    .filter_map(|p| self.get_piece(p))
                    // and get the first piece on the line:
                    .next()
                {
                    // If there aren't any, then there
                    // is no capture
                    None => 0,
                    Some(k) => match k {
                        // If it's a bishop, no capture either
                        PieceKind::Bishop => 0,
                        // If it's a pawn, we capture it
                        PieceKind::Pawn => 1,
                        // If it's another rook, no capture
                        PieceKind::Rook => 0,
                    },
                }
            })
            // ... and we sum the number of captures.
            .sum()
    }

    /// Calculates and returns the total number of captures available
    /// for all the rooks on the board. If two rooks can capture the
    /// same pawn, the capture is counted only once.
    pub fn get_rooks_captures(&self) -> usize {
        let rooks = self.get_rooks_positions();

        rooks // For all rooks
            .iter()
            .map(|start| {
                Direction::all()
                    .iter()
                    .filter_map(|d| {
                        match start
                            .line(*d, self.size)
                            .iter()
                            .filter_map(|p| self.get_piece(p).map(|k| (k, p)))
                            .next()
                        {
                            None => None,
                            Some((k, p)) => match k {
                                PieceKind::Bishop => None,
                                PieceKind::Pawn => Some(*p),
                                PieceKind::Rook => None,
                            },
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .fold(HashSet::new(), |a, b| a.union(&b).copied().collect())
            .len()
    }

    /// Computes the number of pawns the rook can capture in the
    /// board's current configuration, in parallel. This function
    /// assumes that there is only one rook on the board.
    pub fn get_rook_captures_par(&self) -> usize {
        let start = self.get_rook_position();

        Direction::all()
            // We use a parallel iterator here
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
                        PieceKind::Rook => 0,
                    },
                }
            })
            .sum()
    }

    /// Calculates and aggregates the number of captures for all the
    /// rooks on the board. When two rooks can capture the same pawn,
    /// only one capture is counted. The strategy here is to
    /// parallelize on the rooks and not on the 4 directions.
    pub fn get_rooks_captures_par(&self) -> usize {
        let rooks = self.get_rooks_positions();

        rooks
            .par_iter()
            .map(|start| {
                Direction::all()
                    .iter()
                    .filter_map(|d| {
                        match start
                            .line(*d, self.size)
                            .iter()
                            .filter_map(|p| self.get_piece(p).map(|k| (k, p)))
                            .next()
                        {
                            None => None,
                            Some((k, p)) => match k {
                                PieceKind::Bishop => None,
                                PieceKind::Pawn => Some(*p),
                                PieceKind::Rook => None,
                            },
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .reduce(HashSet::new, |a, b| a.union(&b).copied().collect())
            .len()
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
