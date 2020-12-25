use crate::{Board, PieceKind, Position};
use rand::Rng;

pub trait Generator {
    type Output;
    fn generate(&self) -> Self::Output;
}

pub struct BoardGenerator {
    board_size: u32,
    pawn_count: u32,
    bishop_count: u32,
}

impl BoardGenerator {
    pub fn new(board_size: u32) -> Self {
        BoardGenerator {
            board_size,
            pawn_count: (board_size * board_size) / 8,
            bishop_count: (board_size * board_size) / 8,
        }
    }

    pub fn pawn_count(mut self, pawn_count: u32) -> Self {
        self.pawn_count = pawn_count;
        self
    }

    pub fn bishop_count(mut self, bishop_count: u32) -> Self {
        self.bishop_count = bishop_count;
        self
    }
}

impl Generator for BoardGenerator {
    type Output = Board;

    fn generate(&self) -> Board {
        let mut board = Board::new(self.board_size);
        let mut random = rand::thread_rng();

        let positions = Position::generate_unique_positions(
            &mut random,
            self.bishop_count + self.pawn_count + 1,
            self.board_size,
        );

        let pieces: Vec<(PieceKind, Position)> = positions
            .as_slice()
            .chunks(self.bishop_count as usize)
            .enumerate()
            .flat_map(|(i, p)| {
                let kind = match i {
                    0 => PieceKind::Bishop,
                    1 => PieceKind::Pawn,
                    _ => PieceKind::Rook,
                };
                p.iter().map(|p| (kind, *p)).collect::<Vec<_>>()
            })
            .collect();

        board.set_pieces(pieces);
        board
    }
}
