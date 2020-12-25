/// The different chess pieces used in the problem.
#[derive(Debug, Copy, Clone)]
pub enum PieceKind {
    /// The white rook, seeking to capture black pawns
    Rook,
    /// The black pawns, can be captured by the rook
    Pawn,
    /// The white bishop, that can block the path of the white rook
    Bishop,
}
