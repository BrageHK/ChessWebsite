use chess::{Board, Piece, Color, BitBoard};

fn get_piece_by_type(board: &Board, piece: Piece, color: Color) -> BitBoard {
    let piece_bitboard = board.pieces(piece);
    let color_bitboard = board.color_combined(color);
    *piece_bitboard & *color_bitboard
}

/// Very naive utility function
pub(crate) fn eval(board: &Board) -> f32 {
    let mut utility_score = 0.;
    let player = board.side_to_move();
    // pawn points:
    utility_score += get_piece_by_type(&board, Piece::Pawn, player).popcnt() as f32;
    utility_score -= get_piece_by_type(&board, Piece::Pawn, !player).popcnt() as f32;
    // Bishop points:
    utility_score += 3. * get_piece_by_type(&board, Piece::Bishop, player).popcnt() as f32;
    utility_score -= 3. * get_piece_by_type(&board, Piece::Bishop, !player).popcnt() as f32;
    // Knight points:
    utility_score += 3. * get_piece_by_type(&board, Piece::Knight, player).popcnt() as f32;
    utility_score -= 3. * get_piece_by_type(&board, Piece::Knight, !player).popcnt() as f32;
    // Rook points:
    utility_score += 5. * get_piece_by_type(&board, Piece::Rook, player).popcnt() as f32;
    utility_score -= 5. * get_piece_by_type(&board, Piece::Rook, !player).popcnt() as f32;
    // Queen points:
    utility_score += 9. * get_piece_by_type(&board, Piece::Queen, player).popcnt() as f32;
    utility_score -= 9. * get_piece_by_type(&board, Piece::Queen, !player).popcnt() as f32;
    //println!("Score: {}", utility_score);
    utility_score
}