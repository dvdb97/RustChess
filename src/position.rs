use crate::bb_ops;
use crate::bb_ops::coords_to_index;
use crate::rooks;
use crate::bishops;
use crate::knights;
use crate::kings;
use crate::pawns;
use crate::moves::{Move};

use regex::{Regex, Error, RegexSet};

pub const RANKS: u8 = 8;
pub const FILES: u8 = 8;
const NUM_SQUARES: u8 = RANKS * FILES;
const NUM_PIECE_TYPES: u8 = 6;

const NUM_COLORS: u8 = 2;
pub const WHITE: u8 = 0;
pub const BLACK: u8 = 1;

pub const KING: u8 = 5;
pub const QUEEN: u8 = 4;
pub const ROOK: u8 = 3;
pub const BISHOP: u8 = 2;
pub const KNIGHT: u8 = 1;
pub const PAWN: u8 = 0;

const WHITE_PAWNS_INIT: u64 = 0xFF00;
const BLACK_PAWNS_INIT: u64 = 0xFF000000000000;

const WHITE_KNIGHTS_INIT: u64 = 0x42;
const BLACK_KNIGHTS_INIT: u64 = 0x4200000000000000;

const WHITE_BISHOPS_INIT: u64 = 0x24;
const BLACK_BISHOPS_INIT: u64 = 0x2400000000000000;

const WHITE_ROOKS_INIT: u64 = 0x81;
const BLACK_ROOKS_INIT: u64 = 0x8100000000000000;

const WHITE_KINGS_INIT: u64 = 0x10;
const BLACK_KINGS_INIT: u64 = 0x1000000000000000;

const WHITE_QUEENS_INIT: u64 = 0x8;
const BLACK_QUEENS_INIT: u64 = 0x800000000000000;


/// Given a color return the opponent's color.
pub fn flip_color(color: u8) -> u8 {
    return 1 - color;
}


/// Convert the index of a file into the corresponding letter.
pub fn file_to_string(rk: u8) -> Option<char> {
    return match rk {
        0..=7 => Some((0x61u8 + rk) as char),
        _     => None
    };
}


pub fn string_to_file(fl: char) -> Option<u8> {
    return match fl {
        'a'..='h' => Some(fl as u8 - 0x61u8),
        _         => None
    }
}


/// Convert the index of a rank into the corresponding number.
pub fn rank_to_string(fl: u8) -> Option<char> {
    return match fl {
        0..=7 => char::from_digit((fl + 1) as u32, 10),
        _     => None
    }; 
}


pub fn string_to_rank(rk: char) -> Option<u8> {
    return match rk {
        '1'..='8' => rk.to_digit(10).map(|d| (d-1) as u8),
        _         => None
    };
}


/// Convert the index of a square into the square's notation
/// (e.g. a4).
pub fn index_to_string(idx: u8) -> Option<String> {
    let (rk, fl) = bb_ops::index_to_coords(idx);

    return coords_to_string(rk, fl);
}


/// Convert a square's notation into the corresponding index.
pub fn string_to_index(string: String) -> Option<u8> {
    return string_to_coords(string).map(|(rk, fl)| bb_ops::coords_to_index(rk, fl));
}


/// Convert the coords of a square into the square's notation
/// (e.g. a4).
pub fn coords_to_string(rk: u8, fl: u8) -> Option<String> {
    let rk_c = rank_to_string(rk);
    let fl_c = file_to_string(fl);

    return rk_c.zip(fl_c).map(|(r, f)| format!("{}{}", f, r));
}


/// Convert a square's notation into the corresponding coords.
pub fn string_to_coords(string: String) -> Option<(u8, u8)> {
    let mut chars = string.chars();

    let fl = chars.nth(0).and_then(string_to_file);
    let rk = chars.nth(0).and_then(string_to_rank);
    return rk.zip(fl);
}


/// Convert a piece encoded as an integer into the corresponding
/// abbreviation string.
pub fn piece_to_string(piece_type: u8) -> char {
    match piece_type {
        KNIGHT  => 'N',
        BISHOP  => 'B',
        ROOK    => 'R',
        QUEEN   => 'Q',
        KING    => 'K',
        _       => ' '
    }
}


/// Convert a string describing a piece into the corresponding
/// integer.
pub fn string_to_piece(c: char) -> u8 {
    return match c {
        'N' | 'n' => KNIGHT,
        'B' | 'b' => BISHOP,
        'R' | 'r' => ROOK,
        'Q' | 'q' => QUEEN,
        'K' | 'k' => KING,
        _         => PAWN
    };
}

#[derive(Clone)]
pub struct Position {
    pub turn: u8,
    piece_bbs: [u64; 12],
    qs_castle: [bool; 2],
    ks_castle: [bool; 2],
    en_passant: Option<u8>,
    legal_moves: Option<Vec<Move>>
}

impl Position {
    /// Constructors ///
    pub fn empty() -> Position {
        return Position {
            turn: WHITE,
            piece_bbs: [0; 12],
            qs_castle: [true, true],
            ks_castle: [true, true],
            en_passant: None,
            legal_moves: None
        };
    }

    pub fn new(turn: u8, piece_bbs: [u64; 12], qs_castle: [bool; 2], ks_castle: [bool; 2], en_passant: Option<u8>) -> Position {
        return Position {
            turn: turn,
            piece_bbs: piece_bbs,
            qs_castle: qs_castle,
            ks_castle: ks_castle,
            en_passant: en_passant,
            legal_moves: None
        };
    }

    /// Construct a Position object encoding the starting position in regular chess.
    pub fn starting_position() -> Position {
        return Position {
            turn: WHITE,
            piece_bbs: [
                WHITE_PAWNS_INIT,
                WHITE_KNIGHTS_INIT,
                WHITE_BISHOPS_INIT,
                WHITE_ROOKS_INIT,
                WHITE_QUEENS_INIT,
                WHITE_KINGS_INIT,
                BLACK_PAWNS_INIT,
                BLACK_KNIGHTS_INIT,
                BLACK_BISHOPS_INIT,
                BLACK_ROOKS_INIT,
                BLACK_QUEENS_INIT,
                BLACK_KINGS_INIT,
            ],
            qs_castle: [true, true],
            ks_castle: [true, true],
            en_passant: None,
            legal_moves: None
        };
    }

    /// Construct a Position object that represents the position encoded by the given FEN.
    pub fn from_fen(fen: String) -> Option<Position> {
        let mut fields = fen.split(" ");

        // Parse the first part of the FEN that encoded the piece placement in the position.
        let ranks = fields.next().map(|rs| rs.split("/"));
        let piece_bbs = ranks.map(|rs| {
            let mut bbs = [0; 12];

            for (ri, r) in rs.enumerate() {
                let mut fi: u8 = 0;

                for c in r.chars() {
                    if c.is_digit(10) {
                        fi += c.to_digit(10).unwrap() as u8;
                    } else {
                        let piece_idx = string_to_piece(c) + NUM_PIECE_TYPES * if c.is_uppercase() { WHITE } else { BLACK };

                        bbs[piece_idx as usize] |= bb_ops::index_lookup_mask((7 - ri as u8) * 8 + fi);
                        fi += 1;
                    }
                }
            }

            return bbs;
         });

        // Only allow "W", "w", "S", "s" to define what turn it is.
        let turn = fields.next().map(|s| s.to_lowercase())
                                .filter(|s| *s == "w" || *s == "b")
                                .map(|s| if s == "w" { WHITE } else { BLACK });

        // Parse the castling rights for this position.
        let castling = fields.next().map(|s| {
            let mut ks_castle = [s.contains("K"), s.contains("k")];
            let mut qs_castle = [s.contains("Q"), s.contains("q")];

            return (ks_castle, qs_castle);
        });

        // Parse a potential square to capture en passant.
        let en_passant = fields.next().and_then(|s| string_to_coords(s.to_string()))
                                                  .map(|(rk, fl)| coords_to_index(rk, fl));

        return turn.zip(piece_bbs).zip(castling).map(| ((turn, piece_bbs), castling) | {
            return Position::new(turn, piece_bbs, castling.1, castling.0, en_passant);
        });
    }

    /// Board manipulation functions ///

    /// Add a piece of a given type at the square given by the index.
    fn add_piece(&mut self, color: u8, piece_type: u8, idx: u8) {
        let piece_idx = color * NUM_PIECE_TYPES + piece_type;

        self.piece_bbs[piece_idx as usize] = bb_ops::set_idx_bit(self.piece_bbs[piece_idx as usize], idx);
    }

    /// Remove a piece from the square given by the index.
    fn remove_piece(&mut self, idx: u8) {
        for piece_idx in 0..2*NUM_PIECE_TYPES {
            self.piece_bbs[piece_idx as usize] = bb_ops::erase_idx_bit(self.piece_bbs[piece_idx as usize], idx);
        }
    }

    /// Change whose turn it is by flipping the color.
    pub fn flip_turn(&mut self) {
        self.turn = flip_color(self.turn);
    }

    /// Apply a new move to this board. Currently, this function doesn't check the legality of the move
    /// because it's assumed that the given move was generated by the move generator.
    pub fn make_move(&mut self, m: Move) -> Position {
        let mut position = self.clone();

        match m {
            Move::StandardMove(piece_type, origin, target, captures, promotes_to, en_passant) => {
                // Remove the moved piece from the original square.
                position.remove_piece(origin);

                // Check if the piece to put at the target square is identical to the one that originally 
                // was on the origin square.
                let tgt_type = match promotes_to {
                    Some(t) => t,
                    _       => piece_type
                };

                // Add the piece to the target square.
                position.add_piece(position.turn, tgt_type, target);

                // Remove the captured piece from the target square.
                match captures {
                    Some(t) => {
                        position.remove_piece(target);
                    }
                    _       => ()
                };

                position.en_passant = en_passant;
            },
            Move::EnPassant(origin, target) => {
                // Remove the capturing pawn from the original square.
                position.remove_piece(origin);

                // Add the capturing pawn to the new (target) square.
                position.add_piece(position.turn, PAWN, target);

                // Determine the square of the captured pawn. It's one rank further
                // to the center than the square to capture towards.
                let captured_square = match flip_color(position.turn) {
                    WHITE => target + FILES,
                    _     => target - FILES
                };

                // Remove the captured pawn.
                position.remove_piece(captured_square);

                // No en passant possible after this move.
                position.en_passant = None;
            },
            Move::ShortCastle => {
                // Determine the squares the pieces are positioned at / will be 
                // moved to.
                let h =  match position.turn {
                    WHITE => bb_ops::coords_to_index(0, 7),
                    _     => bb_ops::coords_to_index(7, 7)
                };

                let g = h - 1;
                let f = g - 1;
                let e = f - 1;

                // Move the king to his new square.
                position.remove_piece(e);
                position.add_piece(WHITE, KING, g);

                // Move the queen to his new square.
                position.remove_piece(h);
                position.add_piece(WHITE, ROOK, f);

                // Disallow future castling.
                position.remove_castling_rights(position.turn);

                // No en passant possible after this move.
                position.en_passant = None;
            },
            Move::LongCastle => {
                // Determine the squares the pieces are positioned at / will be 
                // moved to.
                let e = match position.turn {
                    WHITE => bb_ops::coords_to_index(0, 4),
                    _     => bb_ops::coords_to_index(7, 4)
                };

                let d = e - 1;
                let c = d - 1;
                let a = c - 2;

                // Move the king to his new square.
                position.remove_piece(e);
                position.add_piece(WHITE, KING, c);

                // Move the queen to his new square.
                position.remove_piece(a);
                position.add_piece(WHITE, ROOK, d);

                position.remove_castling_rights(position.turn);

                // No en passant possible after this move.
                position.en_passant = None;
            }
        }

        position.flip_turn();

        return position;
    }

    /// Returns true if the side specified by the given color still has the right
    /// to castle kingside.
    pub fn can_castle_kingside(&self, color: u8) -> bool {
        return self.ks_castle[color as usize];
    }

    /// Returns true if the side specified by the given color still has the right
    /// to castle queenside.
    pub fn can_castle_queenside(&self, color: u8) -> bool {
        return self.qs_castle[color as usize];
    }

    /// Strips the player with the given color off his castling rights.
    pub fn remove_castling_rights(&mut self, color: u8) {
        self.ks_castle[color as usize] = false;
        self.qs_castle[color as usize] = false;
    }

    /// Restores the castling rights for the player of the given color.
    pub fn restore_castling_rights(&mut self, color: u8) {
        self.ks_castle[color as usize] = true;
        self.qs_castle[color as usize] = true;
    }

    /// Returns true if the king with the given color is checked.
    pub fn is_checked(&self, color: u8) -> bool {
        let attacked_squares = self.get_all_attack_bitboard(flip_color(color));
        let king_bb = self.piece_bbs[(color * NUM_PIECE_TYPES + KING) as usize];

        return attacked_squares & king_bb != 0;
    }

    fn exposes_friendly_king(&mut self, m: Move) -> bool {
        return self.make_move(m).is_checked(self.turn);
    }

    /// Returns the bitboard with the given index.
    pub fn get_bitboard(&self, idx: usize) -> u64 {
        return self.piece_bbs[idx as usize];
    }

    /// Returns a bitboard containing all pieces of a given type and
    /// color.
    pub fn get_piece_bitboard(&self, color: u8, piece_type: u8) -> u64 {
        return self.piece_bbs[(color * NUM_PIECE_TYPES + piece_type) as usize];
    }

    /// Returns a bitboard containing all pieces irrespective of type
    /// for a given color.
    fn get_all_piece_bitboard(&self, color: u8) -> u64 {
        let mut all_pieces = 0;

        for piece_type in 0..NUM_PIECE_TYPES {
            all_pieces |= self.get_piece_bitboard(color, piece_type);
        }

        return all_pieces;
    }

    /// Returns a bitboard marking all squares with 1s that are currently attacked atleast
    /// once by a piece with the given color.
    fn get_all_attack_bitboard(&self, color: u8) -> u64 {
        let pawn_bb = self.get_all_pawn_attacks_bb(color);
        let knight_bb = self.get_all_knight_attacks_bb(color);
        let bishop_bb: u64 = self.get_all_bishop_attacks_bb(color);
        let rook_bb = self.get_all_rook_attacks_bb(color);
        let queen_bb: u64 = self.get_all_queen_attacks_bb(color);
        let king_bb: u64 = self.get_all_king_attacks_bb(color);

        return pawn_bb | knight_bb | bishop_bb | rook_bb | queen_bb | king_bb;
    }

    /// Returns a bitboard containing all friendly pieces for a given color
    /// that could block a sliding piece's movement.
    ///
    /// Equivalent to get_all_piece_bitboard just with a different name
    /// for clarity.
    fn get_friendly_blockers(&self, color: u8) -> u64 {
        return self.get_all_piece_bitboard(color);
    }

    /// Returns a bitboard containing all opponent's pieces for a given color
    /// that could block a sliding piece's movement.
    ///
    /// Equivalent to get_all_piece_bitboard with flipped color just with
    /// a different name for clarity.
    fn get_opponent_blockers(&self, color: u8) -> u64 {
        return self.get_all_piece_bitboard(flip_color(color));
    }

    /// Returns a bitboard containing all pieces on the board that color
    /// block the movement of sliding piece.
    fn get_all_blockers(&self, color: u8) -> u64 {
        return self.get_friendly_blockers(color) | self.get_opponent_blockers(color);
    }

    /// Checks if a given square is occupied by a piece of a given color.
    pub fn is_occupied_by(&self, color: u8, sq: u8) -> bool {
        return bb_ops::index_lookup(self.get_all_blockers(color), sq);
    }

    /// Return the type of the piece at a specific square. Returns None if the square is empty.
    pub fn get_piece_at(&self, color: u8, idx: u8) -> Option<u8> {
        let mut piece = None;

        for piece_type in 0..NUM_PIECE_TYPES {
            if bb_ops::index_lookup(self.piece_bbs[(color * NUM_PIECE_TYPES + piece_type) as usize], idx) {
                piece = Some(piece_type);
                
                break;
            }
        }

        return piece;
    }

    /// Return the type and color of the piece at a specific square. Returns None if the square is empty.
    pub fn get_piece_and_color_at(&self, idx: u8) -> Option<(u8, u8)> {
        let mut piece = None;

        for color in [WHITE, BLACK] {
            for piece_type in 0..NUM_PIECE_TYPES {
                if bb_ops::index_lookup(self.piece_bbs[(color * NUM_PIECE_TYPES + piece_type) as usize], idx) {
                    piece = Some((piece_type, color));
                    
                    break;
                }
            }
        }

        return piece;
    }

    /// Returns the indices of all pieces of a given color and type.
    fn get_piece_indices(&self, color: u8, piece_type: u8) -> Vec<u8> {
        return bb_ops::idx_bitscan(self.get_piece_bitboard(color, piece_type));
    }

    /// Returns a bitboard with 1s marking all squares a pawn of a given color and a
    /// given position can move to (moving and capturing).
    fn get_pawn_moves_bb(&self, color: u8, idx: u8) -> u64 {
        let moves = pawns::get_pawn_moves(color, idx) & !self.get_all_blockers(color);
        let captures = pawns::get_pawn_captures(color, idx) & self.get_opponent_blockers(color);
        let double_steps = pawns::get_pawn_double_steps(color, idx) & !self.get_all_blockers(color);

        return moves | captures | double_steps;
    }

    /// Returns a bitboard with 1s marking all squares a pawn of a given color and a
    /// given position currently controls.
    fn get_pawn_attacks_bb(&self, color: u8, idx: u8) -> u64 {
        return pawns::get_pawn_captures(color, idx);
    }

    /// Returns a bitboard marking all squares a king with a given color and position is currently
    /// attacking.
    fn get_king_attacks_bb(&self, idx: u8) -> u64 {
        return kings::get_king_attacks(idx);
    }

    /// Returns a bitboard marking all squares a king with a given color and position can currently
    /// move to. This function uses the attacks generated by "get_king_attacks_bb" and adds the castling
    /// operation in top of it.  
    fn get_king_moves_bb(&self, color: u8, idx: u8) -> u64 {
        let mut bb = self.get_king_attacks_bb(idx);

        // A king can't move to a square occupied by an allied piece.
        bb &= !self.get_friendly_blockers(color);

        // A king can't move into check.
        bb &= !self.get_all_attack_bitboard(flip_color(color));
        
        return bb;
    }    

    /// Returns a bitboard marking all squares a knight of the given color
    /// and position can move to.
    fn get_knight_attacks_bb(&self, color: u8, idx: u8) -> u64 {
        return knights::get_knight_attacks(idx) & !self.get_friendly_blockers(color);
    }

    /// Combines move bitboard generation for rooks, bishops and queens. It seemed
    /// to be a better solution than having the same code in three separate function.
    fn get_sliding_piece_attacks_bb(&self, color: u8, idx: u8, piece_type: u8) -> u64 {
        let mut bb: u64 = 0;

        let friendly_blockers = self.get_friendly_blockers(color);
        let blockers = friendly_blockers | self.get_opponent_blockers(color);

        // This condition makes sure that both rook and bishop attacks are added
        // if piece_type == QUEEN.
        if piece_type != BISHOP {
            bb |= rooks::get_rook_attacks(idx, blockers);
        }

        // This condition makes sure that both rook and bishop attacks are added
        // if piece_type == QUEEN.
        if piece_type != ROOK {
            bb |= bishops::get_bishop_attacks(idx, blockers);
        }

        bb &= !friendly_blockers;

        return bb;
    }

    /// Returns a bitboard marking all squares a bishop of the given color and position 
    /// can move to.
    fn get_bishop_attacks_bb(&self, color: u8, idx: u8) -> u64 {
        return self.get_sliding_piece_attacks_bb(color, idx, BISHOP);
    }

    /// Returns a bitboard marking all squares a rook of the given color and position 
    /// can move to.
    fn get_rook_attacks_bb(&self, color: u8, idx: u8) -> u64 {
        return self.get_sliding_piece_attacks_bb(color, idx, ROOK);
    }

    /// Returns a bitboard marking all squares a queen of the given color and position 
    /// can move to.
    fn get_queen_attacks_bb(&self, color: u8, idx: u8) -> u64 {
        return self.get_sliding_piece_attacks_bb(color, idx, QUEEN);
    }

    /// Returns a bitboard marking all squares a piece of the given color, type and position 
    /// is attacking.
    fn get_piece_attacks_bb(&self, color: u8, idx: u8, piece_type: u8) -> u64 {
        match piece_type {
            PAWN    => self.get_pawn_attacks_bb(color, idx),
            KNIGHT  => self.get_knight_attacks_bb(color, idx),
            BISHOP  => self.get_bishop_attacks_bb(color, idx),
            ROOK    => self.get_rook_attacks_bb(color, idx),
            QUEEN   => self.get_queen_attacks_bb(color, idx),
            KING    => self.get_king_attacks_bb(idx),
            _       => 0
        }
    }

    /// Checks if a piece of a given type and color at a given position attacks a square.
    fn attacks_square(&self, color: u8, idx: u8, piece_type: u8, sq: u8) -> bool {
        return bb_ops::index_lookup(self.get_piece_attacks_bb(color, idx, piece_type), sq);
    }

    /// Finds all pieces of a given color and type that attack a given square.
    fn get_attackers(&self, color: u8, piece_type: u8, sq: u8) -> Vec<u8> {
        let flipped_color = flip_color(color);

        // Determine a mask for all squares on which an attacker could potentially be.
        // The way of determining this is to find all squares that an opposing color piece
        // positioned on sq could attack.
        let attacker_bb = match piece_type {
            PAWN                  => self.get_pawn_attacks_bb(flipped_color, sq),
            KNIGHT                => self.get_knight_attacks_bb(flipped_color, sq),
            BISHOP | ROOK | QUEEN => self.get_sliding_piece_attacks_bb(flipped_color, sq, piece_type),
            _                     => self.get_king_attacks_bb(sq)
        };

        return bb_ops::idx_bitscan(attacker_bb & self.get_piece_bitboard(color, piece_type))
    }

    /// Finds all pieces of a given color and type that can move to a given square.
    fn can_move_to(&self, color: u8, piece_type: u8, sq: u8) -> Vec<u8> {
        return match piece_type {
            PAWN => {
                let (rk, fl) = bb_ops::index_to_coords(sq);

                if self.is_occupied_by(flip_color(color), sq) {
                    return self.get_attackers(color, piece_type, sq);
                } else {
                    let mut occupiers = bb_ops::idx_bitscan(pawns::get_pawn_moves(flip_color(color), sq) & self.get_piece_bitboard(color, piece_type));

                    let double_step_start_sq = match color {
                        WHITE => bb_ops::coords_to_index(1, fl),
                        _     => bb_ops::coords_to_index(6, fl)
                    };

                    let double_step_rank = 3 + (color * 1);

                    if rk == double_step_rank {
                        match self.get_piece_at(color, double_step_start_sq) {
                            Some(_) => occupiers.push(double_step_start_sq),
                            None => ()
                        };
                    }

                    return occupiers;
                }
            },
            _    => self.get_attackers(color, piece_type, sq)
        };
    }

    /// Returns a bitboard marking all squares a piece with a given color, type
    /// and position can move to.
    fn get_piece_moves_bb(&self, color: u8, idx: u8, piece_type: u8) -> u64 {
        match piece_type {
            PAWN    => self.get_pawn_moves_bb(color, idx),
            KNIGHT  => self.get_knight_attacks_bb(color, idx),
            BISHOP  => self.get_bishop_attacks_bb(color, idx),
            ROOK    => self.get_rook_attacks_bb(color, idx),
            QUEEN   => self.get_queen_attacks_bb(color, idx),
            KING    => self.get_king_moves_bb(color, idx),
            _       => 0
        }
    }

    /// Returns a bitboard marking all squares attacked atleast once by pieces of a given
    /// type and color. 
    fn get_all_piece_attacks_bb(&self, color: u8, piece_type: u8) -> u64 {
        let pieces = self.get_piece_indices(color, piece_type);

        let mut bb: u64 = 0;

        for piece in pieces {
            bb |= self.get_piece_attacks_bb(color, piece, piece_type);
        }

        return bb;
    }

    /// Returns a bitboard marking all squares attacked atleast once by pieces of a given color.
    /// 
    /// TODO: compute this once per position.
    fn get_all_attacks_bb(&self, color: u8) -> u64 {
        let mut bb: u64 = 0;

        for piece_type in [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING] {
            bb |= self.get_all_piece_attacks_bb(color, piece_type);
        }

        return bb;
    }

    /// Returns a bitboard marking all squares attacked atleast once by a pawnm of the
    /// given color.
    fn get_all_pawn_attacks_bb(&self, color: u8) -> u64 {
        return self.get_all_piece_attacks_bb(color, PAWN);
    }

    /// Returns a bitboard marking all squares reachable by a knight of the
    /// given color.
    fn get_all_knight_attacks_bb(&self, color: u8) -> u64 {
        return self.get_all_piece_attacks_bb(color, KNIGHT);
    }

    /// Returns a bitboard marking all squares reachable by a bishop of the
    /// given color.
    fn get_all_bishop_attacks_bb(&self, color: u8) -> u64 {
        return self.get_all_piece_attacks_bb(color, BISHOP);
    }

    /// Returns a bitboard marking all squares reachable by a rook of the
    /// given color.
    fn get_all_rook_attacks_bb(&self, color: u8) -> u64 {
        return self.get_all_piece_attacks_bb(color, ROOK);
    }

    /// Returns a bitboard marking all squares reachable by a queen of the
    /// given color.
    fn get_all_queen_attacks_bb(&self, color: u8) -> u64 {
        return self.get_all_piece_attacks_bb(color, QUEEN);
    }

    /// Returns a bitboard marking all squares reachable by a king of the
    /// given color.
    fn get_all_king_attacks_bb(&self, color: u8) -> u64 {
        return self.get_all_piece_attacks_bb(color, KING);
    }

    /// Computes all legal moves for a pawn of a given color at a given square. 
    fn get_pawn_moves(&mut self, color: u8, idx: u8) -> Vec<Move> {
        let move_bb = self.get_piece_moves_bb(color, idx, PAWN);
        let mut moves = Vec::new();
        
        // Only look at all legal double steps for that pawn.
        let double_step_bb = pawns::get_pawn_double_steps(color, idx);

        // Only look at the promotion moves for the pawn.
        let promotion_bb = move_bb & pawns::get_pawn_promotion_squares(color);

        // Only look at the non-promotion and non-double step moves for the pawn.
        let move_bb = move_bb & !pawns::get_pawn_promotion_squares(color);

        // Add all non-promotion moves.
        for target in bb_ops::idx_bitscan(move_bb) {
            let captures = self.get_piece_at(flip_color(color), target);
            let m = Move::StandardMove(PAWN, idx, target, captures, None, None);

            if !self.exposes_friendly_king(m) {
                moves.push(m);
            }
        }

        // Add all promotion moves.
        for target in bb_ops::idx_bitscan(promotion_bb) {
            let captures = self.get_piece_at(flip_color(color), target);

            // Add a move for each piece the pawn can promote to.
            for prom_tgt in [KNIGHT, BISHOP, ROOK, QUEEN] {
                let m = Move::StandardMove(PAWN, idx, target, captures, Some(prom_tgt), None);

                // Only allow moves that don't expose the king to a check.
                if !self.exposes_friendly_king(m) {
                    moves.push(m);
                }
            } 
        }

        // Add all double step moves.
        for target in bb_ops::idx_bitscan(double_step_bb) {
            let en_passant_square = match self.turn {
                WHITE => target + FILES,
                _     => target - FILES
            };

            let m = Move::StandardMove(PAWN, idx, target, None, None, Some(en_passant_square));

            if !self.exposes_friendly_king(m) {
                moves.push(m);
            }
        }

        return moves;
    }

    /// Computes all legal moves for a King of a given color at a given square. 
    fn get_king_moves(&self, color: u8, idx: u8) -> Vec<Move> {
        let move_bb = self.get_piece_moves_bb(color, idx, KING);
        let mut moves = Vec::new();

        for target in bb_ops::idx_bitscan(move_bb) {
            let captures = self.get_piece_at(flip_color(color), target);

            moves.push(Move::StandardMove(KING, idx, target, captures, None, None));
        }

        // All squares occupied by a piece or attacked by an opposing piece.
        let attacked_blocked_squares = self.get_all_attack_bitboard(flip_color(color)) | self.get_all_blockers(color);

        // All squares that must not be occupied by any piece or attacked by opposing pieces in order for
        // short castling to be legal.
        let ks_castling = kings::get_ks_castling_squares(color);

        if (attacked_blocked_squares & ks_castling) == 0 && self.can_castle_kingside(color) {
            moves.push(Move::ShortCastle);
        }

        // All squares that must not be occupied by any piece or attacked by opposing pieces in order for
        // long castling to be legal.
        let qs_castling = kings::get_qs_castling_squares(color);

        if (attacked_blocked_squares & qs_castling) == 0 && self.can_castle_queenside(color) {
            moves.push(Move::LongCastle);
        }
        
        return moves;
    }

    /// Computes all legal moves of a piece of a given color and type on a given square.
    pub fn get_piece_moves(&mut self, color: u8, idx: u8, piece_type: u8) -> Vec<Move> {
        match piece_type {
            PAWN => self.get_pawn_moves(color, idx),
            KING => self.get_king_moves(color, idx),
            _    => {
                let move_bb = self.get_piece_moves_bb(color, idx, piece_type);
                let mut moves = Vec::new();

                for target in bb_ops::idx_bitscan(move_bb) {
                    let captures = self.get_piece_at(flip_color(color), target);
                    let m = Move::StandardMove(piece_type, idx, target, captures, None, None);

                    // Only allow moves that don't expose the king to a check.  
                    if !self.exposes_friendly_king(m) {
                        moves.push(m);
                    }
                }

                return moves;
            }
        }        
    }

    pub fn get_all_legal_moves(&mut self) -> Vec<Move> {
        return self.get_all_moves(self.turn);
    }

    pub fn get_all_moves(&mut self, color: u8) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for piece_type in [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING] {
            moves.append(&mut self.get_all_piece_moves(color, piece_type))
        }

        return moves;
    }

    pub fn get_all_piece_moves(&mut self, color: u8, piece_type: u8) -> Vec<Move> {
        let pieces = self.get_piece_indices(color, piece_type);
        let mut moves: Vec<Move> = Vec::new();

        for piece in pieces {
            moves.append(&mut self.get_piece_moves(color, piece, piece_type));
        }

        return moves;
    }

    pub fn string_to_move(&self, string: &str) -> Option<Move> {
        let lower_s = string.to_lowercase();
        let string = lower_s.as_str();

        let std_move_pattern = Regex::new(r"(?P<type>[kqrbn])?(?P<origin>[a-h]?[1-8]?)?(?P<captures>x)?(?P<target>[a-h][1-8])(?P<promotes>=[qrbn])?[+#]?").unwrap();
        let long_castle_pattern = Regex::new(r"o-o-o|0-0-0").unwrap();
        let short_castle_pattern = Regex::new(r"o-o|0-0").unwrap();

        let mv = std_move_pattern.captures(string).map(|m| {
            let piece_type = m.name("type").and_then(|m| m.as_str().chars().nth(0)).map_or(PAWN, |m| string_to_piece(m));
            let target = m.name("target").and_then(|m| string_to_index(String::from(m.as_str()))).unwrap();

            let origin = self.can_move_to(self.turn, piece_type, target)[0];
            let captures = m.name("captures").and_then(|_| self.get_piece_at(flip_color(self.turn), target));


            let promotes = None; //m.name("promotes").map(|m| string_to_piece(m.as_str().chars().nth(1).unwrap()));

            return Move::StandardMove(piece_type, origin, target, captures, promotes, None)
        });

        return mv.or(short_castle_pattern.captures(string).map(|_| Move::ShortCastle))
                 .or(long_castle_pattern.captures(string).map(|_| Move::LongCastle));
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        let mut rows: Vec<String> = Vec::new();

        for rk in (0..8).rev() {
            let col_indices = (0..8).map(|fl| bb_ops::coords_to_index(rk, fl));
            let col_strings: Vec<String> = col_indices.map(|idx| {
                return match self.get_piece_and_color_at(idx) {
                    Some((t, c)) =>  {
                        let string = match t {
                            PAWN => String::from("P"),
                            _    => piece_to_string(t).to_string()
                        };

                        if c == BLACK {
                            return string.to_lowercase();
                        }

                        return string;
                    }
                    None         => String::from("_")
                };
            }).collect();

            rows.push(col_strings.join(" "));
        }

        return rows.join("\n");
    }
}
