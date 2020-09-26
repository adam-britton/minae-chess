use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::ops::Index;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Square {
    a8 = 0,  b8 = 1,  c8 = 2,  d8 = 3,  e8 = 4,  f8 = 5,  g8 = 6,  h8 = 7,
    a7 = 8,  b7 = 9,  c7 = 10, d7 = 11, e7 = 12, f7 = 13, g7 = 14, h7 = 15,
    a6 = 16, b6 = 17, c6 = 18, d6 = 19, e6 = 20, f6 = 21, g6 = 22, h6 = 23,
    a5 = 24, b5 = 25, c5 = 26, d5 = 27, e5 = 28, f5 = 29, g5 = 30, h5 = 31,
    a4 = 32, b4 = 33, c4 = 34, d4 = 35, e4 = 36, f4 = 37, g4 = 38, h4 = 39,
    a3 = 40, b3 = 41, c3 = 42, d3 = 43, e3 = 44, f3 = 45, g3 = 46, h3 = 47,
    a2 = 48, b2 = 49, c2 = 50, d2 = 51, e2 = 52, f2 = 53, g2 = 54, h2 = 55,
    a1 = 56, b1 = 57, c1 = 58, d1 = 59, e1 = 60, f1 = 61, g1 = 62, h1 = 63,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_algebraic(*self as u8))
    }
}

fn to_algebraic(pos: u8) -> String {

    let rank = match pos / 8 {
        0 => "8",
        1 => "7",
        2 => "6",
        3 => "5",
        4 => "4",
        5 => "3",
        6 => "2",
        7 => "1",
        _ => "",
    };

    let file = match pos % 8 {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "",
    };

    String::from(file) + rank
}

#[derive(Clone, Copy)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::WhitePawn => write!(f, "P"),
            Piece::BlackPawn => write!(f, "p"),
            Piece::WhiteKnight => write!(f, "N"),
            Piece::BlackKnight => write!(f, "n"),
            Piece::WhiteBishop => write!(f, "B"),
            Piece::BlackBishop => write!(f, "b"),
            Piece::WhiteRook => write!(f, "R"),
            Piece::BlackRook => write!(f, "r"),
            Piece::WhiteQueen => write!(f, "Q"),
            Piece::BlackQueen => write!(f, "q"),
            Piece::WhiteKing => write!(f, "K"),
            Piece::BlackKing => write!(f, "k"),
        }
    }
}

pub struct Board {
    position: [Option<Piece>; 64],
    white_to_move: bool,
    white_kingside_castle_available: bool,
    white_queenside_castle_available: bool,
    black_kingside_castle_available: bool,
    black_queenside_castle_available: bool,
    ep_target_square: Option<Square>,
    half_move_clock: u16,
    full_move_number: u16,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rank = 8;
        let mut file = 0;
        let mut consecutive_empty_squares = 0;
        for piece in self.position.iter() {
            match piece {
                None => {
                    consecutive_empty_squares += 1;
                },
                Some(p) => {
                    if consecutive_empty_squares != 0 {
                        write!(f, "{}", consecutive_empty_squares)
                            .expect("Error displaying board");
                        consecutive_empty_squares = 0;
                    }
                    write!(f, "{}", p).expect("Error displaying board");
                },
            }
            if file < 7 {
                file += 1;
            } else {
                if consecutive_empty_squares != 0 {
                    write!(f, "{}", consecutive_empty_squares)
                        .expect("Error displaying board");
                    consecutive_empty_squares = 0;
                }
                if rank > 1 {
                    write!(f, "/").expect("Error displaying board");
                } else {
                    write!(f, " ").expect("Error displaying board");
                }
                rank -= 1;
                file = 0;
            }
        }
        write!(
            f,
            "{} {}{}{}{}{} {} {} {}",
            if self.white_to_move {"w"} else {"b"},
            if self.white_kingside_castle_available {"K"} else {""},
            if self.white_queenside_castle_available {"Q"} else {""},
            if self.black_kingside_castle_available {"k"} else {""},
            if self.black_queenside_castle_available {"q"} else {""},
            if self.white_kingside_castle_available ||
               self.white_queenside_castle_available ||
               self.black_kingside_castle_available ||
               self.black_queenside_castle_available {""} else {"-"},
            match &self.ep_target_square {
                None => String::from("-"),
                Some(square) => square.to_string(),
            },
            self.half_move_clock,
            self.full_move_number,
        )
    }
}

impl Index<&str> for Board {
    type Output = Option<Piece>;

    fn index(&self, _pos: &str) -> &Self::Output {
        &None
    }
}

impl Index<&String> for Board {
    type Output = Option<Piece>;

    fn index(&self, _pos: &String) -> &Self::Output {
        &None
    }
}

impl Board {

/// Creates a board set to the starting position.
pub fn from_starting_position() -> Board {

    let starting_position: [Option<Piece>; 64] = [
        Some(Piece::BlackRook), Some(Piece::BlackKnight), Some(Piece::BlackBishop), Some(Piece::BlackQueen), Some(Piece::BlackKing), Some(Piece::BlackBishop), Some(Piece::BlackKnight), Some(Piece::BlackRook),
        Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn),
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn),
        Some(Piece::WhiteRook), Some(Piece::WhiteKnight), Some(Piece::WhiteBishop), Some(Piece::WhiteQueen), Some(Piece::WhiteKing), Some(Piece::WhiteBishop), Some(Piece::WhiteKnight), Some(Piece::WhiteRook),
    ];

    Board {
        position: starting_position,
        white_to_move: true,
        white_kingside_castle_available: true,
        white_queenside_castle_available: true,
        black_kingside_castle_available: true,
        black_queenside_castle_available: true,
        ep_target_square: None,
        half_move_clock: 0,
        full_move_number: 1,
    }
}

/// Creates a board from an input FEN.
pub fn from_fen(_fen: &String) -> Board {

    lazy_static! {
        static ref FEN_RE: Regex = Regex::new(
            concat!(
                r"(?P<8>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<7>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<6>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<5>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<4>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<3>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<2>[PRNBQKprnbqk1-8]{1,8})/",
                r"(?P<1>[PRNBQKprnbqk1-8]{1,8}) ",
                r"(?P<turn>[wb]) ",
                r"(?P<castling_availability>-|K?Q?k?q?) ",
                r"(?P<en_passant_target>-|([a-h])([1-8])) ",
                r"(?P<half_move_clock>0|[1-9][0-9]*) ",
                r"(?P<full_move_number>[1-9][0-9]*)"
            )
        ).unwrap();
    }

    Board::from_starting_position()
}

}
