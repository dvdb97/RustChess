use crate::bitboards::bb_ops;
use crate::position;


/// Masks for looking up or setting the bit of a square
/// given by an index.
pub static mut IDX_MASKS: [u64; 64] = [0;64];

/// Masks for masking out all squares that aren't potential
/// blockers for a rook at a certain square.
pub static mut ROOK_MASKS: [u64; 64] = [0; 64];

/// Masks for masking out all squares that aren't potential
/// blockers for a bishop at a certain square.
pub static mut BISHOP_MASKS: [u64; 64] = [0; 64];

/// Masks for looking up squares a king can move to from a
/// given square.
pub static mut KING_MASKS: [u64; 64] = [0; 64];

/// Masks for checking if all squares between the king and the rook
/// are empty such that castling is possible.
pub static KS_CASTLING_MASKS: [u64; 2] = [0x6000000000000000, 0x60];
pub static QS_CASTLING_MASKS: [u64; 2] = [0xE00000000000000, 0xE];

/// Masks for looking up squares a pawn can move to from a
/// given square.
pub static mut PAWN_MOVE_MASKS: [[u64; 64]; 2] = [[0; 64]; 2];

/// Masks for looking up squares a pawn can capture to from a
/// given square.
pub static mut PAWN_CAPTURE_MASKS: [[u64; 64]; 2] = [[0; 64]; 2];

/// Masks for squares a pawn of a given color can promote at.
pub static PAWN_PROMOTION_SQUARES: [u64; 2] = [0xFF00000000000000, 0xFF];

/// Masks for squares where a pawn can move two extra steps.
pub static mut PAWN_DOUBLE_STEP_MASKS: [[u64; 64]; 2] = [[0; 64]; 2];


/// Generates the masks for looking up or setting the bit
/// of a square given by an index.
fn generate_idx_masks() {
    for i in 0..64 {
        unsafe {
            IDX_MASKS[i as usize] = 1 << i;
        }
    }
}

/// Generates the rook mask for a square given by the parameter idx.
fn generate_rook_mask(idx: u8) -> u64 {
    let mut mask: u64 = 0;
    let coords = bb_ops::index_to_coords(idx);

    for i in 0..64 {
        let curr_coords = bb_ops::index_to_coords(i);

        if (curr_coords.0 == coords.0) ^ (curr_coords.1 == coords.1) {
            mask = bb_ops::set_idx_bit(mask, i);
        }
    }

    return mask;
}

/// Generates the rook masks for all squares on the board and stores
/// them in the static array ROOK_MASKS.
fn generate_rook_masks() {
    for i in 0..64 {
        unsafe {
            ROOK_MASKS[i as usize] = generate_rook_mask(i);
        }
    }
}

/// Generates the bishop mask for a square given by the parameter idx.
fn generate_bishop_mask(idx: u8) -> u64 {
    let mut mask: u64 = 0;
    let coords = bb_ops::index_to_coords(idx);

    for i in 0..64 {
        let curr_coords = bb_ops::index_to_coords(i);
        let dy = (curr_coords.0 as i16) - (coords.0 as i16);
        let dx = (curr_coords.1 as i16) - (coords.1 as i16);

        if dx.abs() == dy.abs() && dx != 0 {
            mask = bb_ops::set_idx_bit(mask, i);
        }
    }

    return mask;
}

/// Generates the bishop masks for all squares on the board and stores
/// them in the static array BISHOP_MASKS.
fn generate_bishop_masks() {
    for i in 0..64 {
        unsafe {
            BISHOP_MASKS[i as usize] = generate_bishop_mask(i);
        }
    }
}

/// Generates the king mask for a square given by the parameter idx.
fn generate_king_mask(idx: u8) -> u64 {
    let mask: u64 = 0;
    let (y, x) = bb_ops::index_to_coords(idx);

    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            let ny = (y as i8) + dy;
            let nx = (x as i8) + dx;

            if bb_ops::is_legal_square(ny, nx) {
                bb_ops::set_coords_bit(mask, ny as u8, nx as u8);
            }
        }
    }

    return mask;
}

/// Generates the king masks for all squares on the board and stores
/// them in the static array KING_MASKS.
fn generate_king_masks() {
    for i in 0..64 {
        unsafe {
            KING_MASKS[i as usize] = generate_king_mask(i);
        }
    }
}

/// Generates the pawns masks for all squares on the board. This involves move masks
/// and capture masks.
fn generate_pawn_masks() {
    for c in [position::WHITE, position::BLACK] {
        for i in 0..64 {
            let (row, col) = bb_ops::index_to_coords(i);

            // The next rank for the pawn to move to.
            let next_row = ((row + 1) as i8) + (c as i8) * (-2i8);

            // Check if the next square is inside the board.
            if next_row >= 0 && next_row < 8 {
                unsafe {
                    let idx = bb_ops::coords_to_index(next_row as u8, col);

                    PAWN_MOVE_MASKS[c as usize][i as usize] = IDX_MASKS[idx as usize];
                }

                // Assemble a mask for potential capture moves.
                let mut attack_mask: u64 = 0;

                // Set the bits on the left and right side of the pawn's front to 1.
                for dx in [-1i8, 1i8] {
                    if (col as i8) + dx >= 0 && (col as i8) + dx < 8 {
                        let next_col = (col as i8 + dx) as u8;

                        let idx = bb_ops::coords_to_index(next_row as u8, next_col);

                        unsafe {
                            attack_mask |= IDX_MASKS[idx as usize];
                        }
                    }
                }
                unsafe {
                    PAWN_CAPTURE_MASKS[c as usize][i as usize] = attack_mask;
                }
            }
        }
    }

    for col in 0..8 {
        unsafe {
            PAWN_DOUBLE_STEP_MASKS[position::WHITE as usize][(position::FILES + col) as usize] = bb_ops::coords_lookup_mask(3, col);
            PAWN_DOUBLE_STEP_MASKS[position::BLACK as usize][(6 * position::FILES + col) as usize] = bb_ops::coords_lookup_mask(4, col);
        }
    }
}


/// Generates all patterns that are used for piece attacks.
/// The masks are precomputed and stored static arrays in order to save
/// computation time during move generation.
pub fn generate_patterns() {
    generate_idx_masks();
    generate_rook_masks();
    generate_bishop_masks();
    generate_king_masks();
    generate_pawn_masks();
}

/// The attack patterns for knights on all squares on the board.
pub const KNIGHT_MASKS: [u64; 64] = [
    0x20400,
    0x50800,
    0xa1100,
    0x142200,
    0x284400,
    0x508800,
    0xa01000,
    0x402000,
    0x2040004,
    0x5080008,
    0xa110011,
    0x14220022,
    0x28440044,
    0x50880088,
    0xa0100010,
    0x40200020,
    0x204000402,
    0x508000805,
    0xa1100110a,
    0x1422002214,
    0x2844004428,
    0x5088008850,
    0xa0100010a0,
    0x4020002040,
    0x20400040200,
    0x50800080500,
    0xa1100110a00,
    0x142200221400,
    0x284400442800,
    0x508800885000,
    0xa0100010a000,
    0x402000204000,
    0x2040004020000,
    0x5080008050000,
    0xa1100110a0000,
    0x14220022140000,
    0x28440044280000,
    0x50880088500000,
    0xa0100010a00000,
    0x40200020400000,
    0x204000402000000,
    0x508000805000000,
    0xa1100110a000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xa0100010a0000000,
    0x4020002040000000,
    0x400040200000000,
    0x800080500000000,
    0x1100110a00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010a000000000,
    0x2000204000000000,
    0x4020000000000,
    0x8050000000000,
    0x110a0000000000,
    0x22140000000000,
    0x44280000000000,
    0x88500000000000,
    0x10a00000000000,
    0x20400000000000,
];
