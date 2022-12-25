use crate::patterns;


pub fn get_king_attacks(sq: u8) -> u64 {
    unsafe {
        return patterns::KING_MASKS[sq as usize];
    }
}


pub fn get_ks_castling_squares(color: u8) -> u64 {
    return patterns::KS_CASTLING_MASKS[color as usize];
}


pub fn get_qs_castling_squares(color: u8) -> u64 {
    return patterns::QS_CASTLING_MASKS[color as usize];
}