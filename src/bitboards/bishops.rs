use crate::bitboards::bb_ops;
use std::cmp;


/// Computes all squares attacked by a bishop on the given square
/// by taking all possible blockerss in consideration. Returns a attack
/// map with all attacked squares set to 1.
///
/// Code adapted from: https://www.chessprogramming.org/Looking_for_Magics
fn bishop_attacks(sq: u8, blockers: u64) -> u64 {
    let mut result: u64 = 0;

    let (rk, fl) = bb_ops::index_to_coords(sq);

    for d in 1..cmp::min(8-rk, 8-fl) {
        result = bb_ops::set_coords_bit(result, rk+d, fl+d);
        if bb_ops::coords_lookup(blockers, rk+d, fl+d) {
            break;
        }
    }

    for d in 1..cmp::min(8-rk, fl+1) {
        result = bb_ops::set_coords_bit(result, rk+d, fl-d);
        if bb_ops::coords_lookup(blockers, rk+d, fl-d) {
            break;
        }
    }

    for d in 1..cmp::min(rk+1, 8-fl) {
        result = bb_ops::set_coords_bit(result, rk-d, fl+d);
        if bb_ops::coords_lookup(blockers, rk-d, fl+d) {
            break;
        }
    }

    for d in 1..cmp::min(rk+1, fl+1) {
        result = bb_ops::set_coords_bit(result, rk-d, fl-d);
        if bb_ops::coords_lookup(blockers, rk-d, fl-d) {
            break;
        }
    }

    return result;
}

pub fn get_bishop_attacks(sq: u8, blockers: u64) -> u64 {
    return bishop_attacks(sq, blockers);
}
