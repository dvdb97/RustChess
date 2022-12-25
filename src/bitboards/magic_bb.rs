use crate::bb_ops;
use crate::patterns;
use crate::std::vec;

// Magic numbers for all rook positions;
pub static ROOK_MAGICS: [u32; 64] = [0; 64];

// Magic numbers for all bishop positions;
pub static BISHOP_MAGICS: [u32; 64] = [0; 64];

fn find_rook_magic(idx: u8) -> u64 {
    let block_att_tble = vec::Vec::new();

    
}

fn find_rook_magics() {
    for i in 0..64 {
        unsafe {
            ROOK_MAGICS[i] = find_rook_magic(i);
        }
    }
}

fn find_bishop_magic(idx: u8) {}

fn find_bishop_magics() {
    for i in 0..64 {
        unsafe {
            BISHOP_MAGICS[i] = find_bishop_magic(i);
        }
    }
}

pub fn find_magics() {
    find_rook_magics();
    find_bishop_magics();
}
