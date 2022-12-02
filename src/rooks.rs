use crate::bb_ops;

/// Computes all squares attacked by a rook on the given square
/// by taking all possible blockers in consideration. Returns a attack
/// map with all attacked squares set to 1.
///
/// Code adapted from: https://www.chessprogramming.org/Looking_for_Magics
fn rook_attacks(sq: u8, blockers: u64) -> u64 {
    let mut result: u64 = 0;

    let (rk, fl) = bb_ops::index_to_coords(sq);

    for r in rk..8 {
        result = bb_ops::set_coords_bit(result, r, fl);
        if bb_ops::coords_lookup(blockers, r, fl) {
            break;
        }
    }

    for r in (0..rk).rev() {
        result = bb_ops::set_coords_bit(result, r, fl);
        if bb_ops::coords_lookup(blockers, r, fl) {
            break;
        }
    }

    for f in fl..8 {
        result = bb_ops::set_coords_bit(result, rk, f);
        if bb_ops::coords_lookup(blockers, rk, f) {
            break;
        }
    }

    for f in (0..fl).rev() {
        result = bb_ops::set_coords_bit(result, rk, f);
        if bb_ops::coords_lookup(blockers, rk, f) {
            break;
        }
    }

    return result;
}

pub fn get_rook_attacks(sq: u8, blockers: u64) -> u64 {
    return rook_attacks(sq, blockers);
}
