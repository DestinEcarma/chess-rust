use crate::color::{Color, Colors};

pub type Piece = usize;

pub struct Pieces;
impl Pieces {
	pub const KING: Piece = 0;
	pub const PAWN: Piece = 1;
	pub const KNIGHT: Piece = 2;
	pub const BISHOP: Piece = 3;
	pub const ROOK: Piece = 4;
	pub const QUEEN: Piece = 5;
	pub const EMPTY: Piece = 6;

	pub const SIZE: usize = 6;

	pub const PROMOTIONAL: [Piece; 4] = [Self::QUEEN, Self::ROOK, Self::BISHOP, Self::KNIGHT];

	pub const VALUES: [u16; Self::SIZE] = [0, 100, 320, 310, 500, 900];

	pub fn from_char(value: char) -> Piece {
		match value {
			'K' | 'k' => Self::KING,
			'P' | 'p' => Self::PAWN,
			'N' | 'n' => Self::KNIGHT,
			'B' | 'b' => Self::BISHOP,
			'R' | 'r' => Self::ROOK,
			'Q' | 'q' => Self::QUEEN,
			_ => panic!("Piece not recognized, got: {value}"),
		}
	}

	pub fn to_char(value: Piece) -> char {
		match value {
			Self::KING => 'k',
			Self::PAWN => 'p',
			Self::KNIGHT => 'n',
			Self::BISHOP => 'b',
			Self::ROOK => 'r',
			Self::QUEEN => 'q',
			_ => panic!("Piece not recognized, got: {value}"),
		}
	}

	pub fn to_string(piece: Piece, color: Color) -> String {
		(match color {
			Colors::WHITE => Self::to_char(piece).to_ascii_uppercase(),
			Colors::BLACK => Self::to_char(piece).to_ascii_lowercase(),
			_ => panic!("Color not recognized, got: {color}"),
		})
		.to_string()
	}
}
