mod bitboards;
mod position;
mod moves;

use std::env;

use crate::position::Position;
use crate::bitboards::patterns;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    patterns::generate_patterns();

    let mut pos = position::Position::starting_position();
    println!("{}\n", pos.to_string());

    for m in ["e4", "c5", "Nf3", "d6", "d4", "cxd4", "Nxd4", "Nf6", "Nc3", "a6", "Be3", "e6", "Qd2", "Be7", "O-O-O"] {
        let c = match pos.turn {
            position::WHITE => "White",
            _     => "Black"
        };

        println!("Turn: {}", c);

        let m = pos.string_to_move(m).unwrap();
        pos = pos.make_move(m);
        println!("After {}:\n{}\n", m.to_string(), pos.to_string());
    }

    let mut pos = position::Position::starting_position();

    for m in ["e4", "e5", "Ke2", "Ke7", "Ke1", "Ke8", "Ke2"] {
        let m = pos.string_to_move(m).unwrap();
        pos = pos.make_move(m);
        
        println!("After {}:\n{}\n", m.to_string(), pos.to_string());
    }

    println!("Is threefold repetition: {}", pos.is_threefold_repetition());

    let mut position = Position::from_fen("r1b1kb1r/3p1ppp/p1n1p1n1/qp1N2B1/4P3/1B3N2/PP3PPP/R2QR1K1 b kq - 7 11".to_string()).expect("Loading FEN failed!");
    println!("From FEN: \n\n{}", position.to_string());

    print!("\n\nLegal Moves:");
    for m in position.get_all_legal_moves() {
        print!("{}, ", m.to_string());
    }

    // TODO: When castling the king isn't properly removed.
}
