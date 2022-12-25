use crate::patterns;


pub fn get_knight_attacks(sq: u8) -> u64 {
    unsafe {
        return patterns::KNIGHT_MASKS[sq as usize];
    }
}
