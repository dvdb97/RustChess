use crate::patterns;

pub fn set_idx_bit(bb: u64, idx: u8) -> u64 {
    return bb | index_lookup_mask(idx);
}

pub fn erase_idx_bit(bb: u64, idx: u8) -> u64 {
    return bb & (!index_lookup_mask(idx));
}

pub fn set_coords_bit(bb: u64, y: u8, x: u8) -> u64 {
    return set_idx_bit(bb, coords_to_index(y, x));
}

pub fn erase_coords_bit(bb: u64, y: u8, x: u8) -> u64 {
    return erase_idx_bit(bb, coords_to_index(y, x));
}

pub fn set_idx_bits(mut bb: u64, idxs: Vec<u8>) -> u64 {
    for idx in idxs {
        bb = set_idx_bit(bb, idx);
    }

    return bb;
}

pub fn index_lookup_mask(idx: u8) -> u64 {
    unsafe{
        return patterns::IDX_MASKS[idx as usize];
    }
}

pub fn coords_lookup_mask(y: u8, x: u8) -> u64 {
    return index_lookup_mask(coords_to_index(y, x));
}

pub fn index_lookup(bb: u64, idx: u8) -> bool {
    return bb & index_lookup_mask(idx) != 0;
}

pub fn coords_lookup(bb: u64, y: u8, x: u8) -> bool {
    return index_lookup(bb, coords_to_index(y, x));
}

pub fn idx_bitscan(bb: u64) -> Vec<u8> {
    let mut idxs = Vec::new();

    for idx in 0..64 {
        if index_lookup(bb, idx) {
            idxs.push(idx);
        }
    }

    return idxs;
}

pub fn coords_bitscan(bb: u64) -> Vec<(u8, u8)> {
    let mut coords = Vec::new();

    for y in 0..8 {
        for x in 0..8 {
            if coords_lookup(bb, y, x) {
                coords.push((y, x));
            }
        }
    }

    return coords;
}

pub fn coords_to_index(y: u8, x: u8) -> u8 {
    return y * 8 + x;
}

pub fn index_to_coords(idx: u8) -> (u8, u8) {
    return (idx / 8, idx % 8);
}

pub fn is_legal_square(y: i8, x: i8) -> bool {
    return y >= 0 && y < 8 && x >= 0 && x < 8;
}

pub fn print_bitboard(bb: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            if coords_lookup(bb, rank, file) {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!("");
    }
}
