use crate::{Board, PieceKind, Position};

pub trait Generator {
    type Output;
    fn generate(&self) -> Self::Output;
}

pub struct BoardGenerator {
    board_size: u32,
    pawn_count: u32,
    bishop_count: u32,
    rook_count: u32,
}

impl BoardGenerator {
    pub fn new(board_size: u32) -> Self {
        BoardGenerator {
            board_size,
            pawn_count: (board_size * board_size) / 8,
            bishop_count: (board_size * board_size) / 8,
            rook_count: (board_size * board_size) / 8,
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

    pub fn rook_count(mut self, rook_count: u32) -> Self {
        self.rook_count = rook_count;
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
            self.bishop_count + self.pawn_count + self.rook_count,
            self.board_size,
        );

        let pieces = &[
            (PieceKind::Bishop, &positions[0..self.bishop_count as usize]),
            (
                PieceKind::Pawn,
                &positions
                    [self.bishop_count as usize..(self.bishop_count + self.pawn_count) as usize],
            ),
            (
                PieceKind::Rook,
                &positions[(self.bishop_count + self.pawn_count) as usize..],
            ),
        ];

        board.set_pieces(
            pieces
                .into_iter()
                .flat_map(|(k, ps)| ps.iter().map(|p| (*k, *p)).collect::<Vec<_>>())
                .collect(),
        );
        board
    }
}
