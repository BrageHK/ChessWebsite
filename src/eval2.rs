use chess::{BitBoard, Board, Color, Piece};

/// Gets the bitboard of all pieces of a certain type and color
fn get_piece_bitboard(board: &Board, piece: Piece, color: Color) -> BitBoard {
    board.pieces(piece) & board.color_combined(color)
}

/// Mirrors the index vertically for Black's perspective
fn mirror_index(index: usize) -> usize {
    63 - index
}

/// Piece-Square Tables (PSTs) with smaller values
const PAWN_PST: [f32; 64] = [
    // A1 to H1
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    // A2 to H2
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    // A3 to H3
    0.5, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, 0.5,
    // A4 to H4
    0.5, 0.5, 1.5, 1.3, 1.3, 1.5, 0.5, 0.5,
    // A5 to H5
    0.5, 0.5, 1.3, 1.3, 1.3, 1.5, 0.5, 0.5,
    // A6 to H6
    0.5, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, 0.5,
    // A7 to H7
    0.5, 0.9, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    // A8 to H8
    0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0,
];

const KNIGHT_PST: [f32; 64] = [
    // A1 to H1
    -1.5, -1.0, -0.5, -0.5, -0.5, -0.5, -1.0, -1.5,
    // A2 to H2
    -1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0,
    // A3 to H3
    -0.5, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -0.5,
    // A4 to H4
    -0.5, 1.0, 2.0, 2.5, 2.5, 2.0, 1.0, -0.5,
    // A5 to H5
    -0.5, 1.0, 2.0, 2.5, 2.5, 2.0, 1.0, -0.5,
    // A6 to H6
    -0.5, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -0.5,
    // A7 to H7
    -1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0,
    // A8 to H8
    -1.5, -1.0, -0.5, -0.5, -0.5, -0.5, -1.0, -1.5,
];

const BISHOP_PST: [f32; 64] = [
    // A1 to H1
    -1.0, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -1.0,
    // A2 to H2
    -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -0.5,
    // A3 to H3
    -0.5, 1.0, 1.0, 1.5, 1.5, 1.0, 1.0, -0.5,
    // A4 to H4
    -0.5, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -0.5,
    // A5 to H5
    -0.5, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -0.5,
    // A6 to H6
    -0.5, 1.0, 1.0, 1.5, 1.5, 1.0, 1.0, -0.5,
    // A7 to H7
    -0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5,
    // A8 to H8
    -1.0, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -1.0,
];

const ROOK_PST: [f32; 64] = [
    // A1 to H1
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    // A2 to H2
    0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5,
    // A3 to H3
    -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
    // A4 to H4
    -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
    // A5 to H5
    -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
    // A6 to H6
    -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
    // A7 to H7
    -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
    // A8 to H8
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

const QUEEN_PST: [f32; 64] = [
    // A1 to H1
    -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0,
    // A2 to H2
    -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0,
    // A3 to H3
    -1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0,
    // A4 to H4
    -0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5,
    // A5 to H5
    -0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5,
    // A6 to H6
    -1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0,
    // A7 to H7
    -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0,
    // A8 to H8
    -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0,
];

const KING_PST: [f32; 64] = [
    // A1 to H1
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    // A2 to H2
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    // A3 to H3
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    // A4 to H4
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    // A5 to H5
    -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0,
    // A6 to H6
    -1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0,
    // A7 to H7
    2.0,  2.0,  0.0,  0.0,  0.0,  0.0,  2.0,  2.0,
    // A8 to H8
    2.0,  3.0,  1.0,  0.0,  0.0,  1.0,  3.0,  2.0,
];

/// Evaluation function considering both player's and opponent's pieces
pub fn eval(board: &Board) -> f32 {
    let mut utility_score = 0.0;

    // The side to move is considered the maximizing player
    let our_color = board.side_to_move();
    let their_color = !our_color;

    // Evaluate our pieces (positive score)
    utility_score += evaluate_side(board, our_color);

    // Evaluate opponent's pieces (negative score)
    utility_score -= evaluate_side(board, their_color);

    utility_score
}

/// Evaluates all pieces of a given color
fn evaluate_side(board: &Board, color: Color) -> f32 {
    let mut score = 0.0;

    // Pawns
    let pawns = get_piece_bitboard(board, Piece::Pawn, color);
    score += evaluate_pieces(pawns, Piece::Pawn, color);

    // Knights
    let knights = get_piece_bitboard(board, Piece::Knight, color);
    score += evaluate_pieces(knights, Piece::Knight, color);

    // Bishops
    let bishops = get_piece_bitboard(board, Piece::Bishop, color);
    score += evaluate_pieces(bishops, Piece::Bishop, color);

    // Rooks
    let rooks = get_piece_bitboard(board, Piece::Rook, color);
    score += evaluate_pieces(rooks, Piece::Rook, color);

    // Queens
    let queens = get_piece_bitboard(board, Piece::Queen, color);
    score += evaluate_pieces(queens, Piece::Queen, color);

    // Kings
    let kings = get_piece_bitboard(board, Piece::King, color);
    score += evaluate_pieces(kings, Piece::King, color);

    score
}

/// Evaluates pieces of a given type and color
fn evaluate_pieces(bitboard: BitBoard, piece: Piece, color: Color) -> f32 {
    let mut score = 0.0;

    let mut bits = bitboard;

    while bits != BitBoard(0) {
        let square = bits.to_square();
        let index = square.to_index() as usize;

        let pst_score = get_pst_value(piece, color, index);
        score += pst_score;

        // Remove the piece from bits
        bits ^= BitBoard::from_square(square);
    }

    score
}

/// Retrieves the PST value for a piece on a given square
fn get_pst_value(piece: Piece, color: Color, index: usize) -> f32 {
    let idx = if color == Color::White {
        index
    } else {
        mirror_index(index)
    };

    match piece {
        Piece::Pawn => PAWN_PST[idx],
        Piece::Knight => KNIGHT_PST[idx],
        Piece::Bishop => BISHOP_PST[idx],
        Piece::Rook => ROOK_PST[idx],
        Piece::Queen => QUEEN_PST[idx],
        Piece::King => KING_PST[idx],
    }
}
