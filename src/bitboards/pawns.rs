use crate::patterns;


pub fn get_pawn_moves(color: u8, sq: u8) -> u64 {
    unsafe {
        return patterns::PAWN_MOVE_MASKS[color as usize][sq as usize];
    }
}


pub fn get_pawn_captures(color: u8, sq: u8) -> u64 {
    unsafe {
        return patterns::PAWN_CAPTURE_MASKS[color as usize][sq as usize];
    }
}


pub fn get_pawn_promotion_squares(color: u8) -> u64 {
    return patterns::PAWN_PROMOTION_SQUARES[color as usize];
}


pub fn get_pawn_double_steps(color: u8, sq: u8) -> u64 {
    unsafe {
        return patterns::PAWN_DOUBLE_STEP_MASKS[color as usize][sq as usize];
    } 
} 