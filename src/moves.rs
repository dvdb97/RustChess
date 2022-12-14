use crate::position;

#[derive(Clone, Copy)]
pub enum Move {
    StandardMove(u8, u8, u8, Option<u8>, Option<u8>, Option<u8>),
    ShortCastle,
    LongCastle
}


impl ToString for Move {
    fn to_string(&self) -> String {
        match self {
            Move::StandardMove(piece_type, origin, target, captures, promotes_to, _) => {
                let origin = position::index_to_string(*origin).unwrap();
                let target = position::index_to_string(*target).unwrap();

                let piece_type = position::piece_to_string(*piece_type);

                let captures = match captures {
                    Some(_) => String::from("x"),
                    _       => String::from(":")
                };

                let promotes_to = match promotes_to {
                    Some(t) => format!("={}", position::piece_to_string(*t)),
                    _       => String::from("")
                };

                return format!("{piece}{origin}{captures}{target}{promote}", piece=piece_type, origin=origin, captures=captures, target=target, promote=promotes_to);
            },
            Move::ShortCastle => String::from("O-O"),
            Move::LongCastle => String::from("O-O-O")
        } 
    }
}