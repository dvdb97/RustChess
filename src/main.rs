use std::env;

mod bb_ops;
mod position;
mod patterns;
mod rooks;
mod bishops;
mod knights;
mod kings;
mod moves;
mod pawns;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    patterns::generate_patterns();

    let mut pos = position::Position::starting_position();
    println!("{}\n", pos.to_string());

    for m in ["e4", "e5", "Nf3", "Nc6", "Bb5", "Nf6", "O-O"] {
        let c = match pos.turn {
            position::WHITE => "White",
            _     => "Black"
        };

        println!("Turn: {}", c);

        let m = pos.string_to_move(m).unwrap();
        pos = pos.make_move(m);
        println!("After {}:\n{}\n", m.to_string(), pos.to_string());
    }

    // TODO: Swap king and queen in the starting position

    // TODO: When castling the king isn't properly removed.
}
