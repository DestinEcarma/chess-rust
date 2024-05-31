use core::panic;

use super::{Square, Squares};

#[allow(dead_code)]
const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

impl Squares {
	pub const A1: Square = 0;
	pub const B1: Square = 1;
	pub const C1: Square = 2;
	pub const D1: Square = 3;
	pub const E1: Square = 4;
	pub const F1: Square = 5;
	pub const G1: Square = 6;
	pub const H1: Square = 7;
	pub const A2: Square = 8;
	pub const B2: Square = 9;
	pub const C2: Square = 10;
	pub const D2: Square = 11;
	pub const E2: Square = 12;
	pub const F2: Square = 13;
	pub const G2: Square = 14;
	pub const H2: Square = 15;
	pub const A3: Square = 16;
	pub const B3: Square = 17;
	pub const C3: Square = 18;
	pub const D3: Square = 19;
	pub const E3: Square = 20;
	pub const F3: Square = 21;
	pub const G3: Square = 22;
	pub const H3: Square = 23;
	pub const A4: Square = 24;
	pub const B4: Square = 25;
	pub const C4: Square = 26;
	pub const D4: Square = 27;
	pub const E4: Square = 28;
	pub const F4: Square = 29;
	pub const G4: Square = 30;
	pub const H4: Square = 31;
	pub const A5: Square = 32;
	pub const B5: Square = 33;
	pub const C5: Square = 34;
	pub const D5: Square = 35;
	pub const E5: Square = 36;
	pub const F5: Square = 37;
	pub const G5: Square = 38;
	pub const H5: Square = 39;
	pub const A6: Square = 40;
	pub const B6: Square = 41;
	pub const C6: Square = 42;
	pub const D6: Square = 43;
	pub const E6: Square = 44;
	pub const F6: Square = 45;
	pub const G6: Square = 46;
	pub const H6: Square = 47;
	pub const A7: Square = 48;
	pub const B7: Square = 49;
	pub const C7: Square = 50;
	pub const D7: Square = 51;
	pub const E7: Square = 52;
	pub const F7: Square = 53;
	pub const G7: Square = 54;
	pub const H7: Square = 55;
	pub const A8: Square = 56;
	pub const B8: Square = 57;
	pub const C8: Square = 58;
	pub const D8: Square = 59;
	pub const E8: Square = 60;
	pub const F8: Square = 61;
	pub const G8: Square = 62;
	pub const H8: Square = 63;

	// the purpose of this function is to initialize the constant Notations
	#[cfg(debug_assertions)]
	pub fn _init_square_notations() {
		for r in Self::RANKS {
			for f in Self::FILES {
				let sq = Self::from_rank_file(r, f);

				let notation = format!("{}{}", FILES[f].to_ascii_uppercase(), r + 1);

				println!("pub const {notation}: Square = {sq};");
			}
		}
	}

	pub fn from_notation(value: &str) -> Square {
		match value {
			"a1" | "A1" => Self::A1,
			"b1" | "B1" => Self::B1,
			"c1" | "C1" => Self::C1,
			"d1" | "D1" => Self::D1,
			"e1" | "E1" => Self::E1,
			"f1" | "F1" => Self::F1,
			"g1" | "G1" => Self::G1,
			"h1" | "H1" => Self::H1,
			"a2" | "A2" => Self::A2,
			"b2" | "B2" => Self::B2,
			"c2" | "C2" => Self::C2,
			"d2" | "D2" => Self::D2,
			"e2" | "E2" => Self::E2,
			"f2" | "F2" => Self::F2,
			"g2" | "G2" => Self::G2,
			"h2" | "H2" => Self::H2,
			"a3" | "A3" => Self::A3,
			"b3" | "B3" => Self::B3,
			"c3" | "C3" => Self::C3,
			"d3" | "D3" => Self::D3,
			"e3" | "E3" => Self::E3,
			"f3" | "F3" => Self::F3,
			"g3" | "G3" => Self::G3,
			"h3" | "H3" => Self::H3,
			"a4" | "A4" => Self::A4,
			"b4" | "B4" => Self::B4,
			"c4" | "C4" => Self::C4,
			"d4" | "D4" => Self::D4,
			"e4" | "E4" => Self::E4,
			"f4" | "F4" => Self::F4,
			"g4" | "G4" => Self::G4,
			"h4" | "H4" => Self::H4,
			"a5" | "A5" => Self::A5,
			"b5" | "B5" => Self::B5,
			"c5" | "C5" => Self::C5,
			"d5" | "D5" => Self::D5,
			"e5" | "E5" => Self::E5,
			"f5" | "F5" => Self::F5,
			"g5" | "G5" => Self::G5,
			"h5" | "H5" => Self::H5,
			"a6" | "A6" => Self::A6,
			"b6" | "B6" => Self::B6,
			"c6" | "C6" => Self::C6,
			"d6" | "D6" => Self::D6,
			"e6" | "E6" => Self::E6,
			"f6" | "F6" => Self::F6,
			"g6" | "G6" => Self::G6,
			"h6" | "H6" => Self::H6,
			"a7" | "A7" => Self::A7,
			"b7" | "B7" => Self::B7,
			"c7" | "C7" => Self::C7,
			"d7" | "D7" => Self::D7,
			"e7" | "E7" => Self::E7,
			"f7" | "F7" => Self::F7,
			"g7" | "G7" => Self::G7,
			"h7" | "H7" => Self::H7,
			"a8" | "A8" => Self::A8,
			"b8" | "B8" => Self::B8,
			"c8" | "C8" => Self::C8,
			"d8" | "D8" => Self::D8,
			"e8" | "E8" => Self::E8,
			"f8" | "F8" => Self::F8,
			"g8" | "G8" => Self::G8,
			"h8" | "H8" => Self::H8,
			_ => panic!("Notation not recognized, got: {value}"),
		}
	}

	// The purpose of this function is to initialize the from_notation function
	#[cfg(debug_assertions)]
	pub fn _init_from_notation() {
		for r in Self::RANKS {
			for f in Self::FILES {
				let notation = format!("{}{}", FILES[f], r + 1);

				println!(
					r#""{notation}" | "{0}" => Self::{0},"#,
					notation.to_uppercase()
				)
			}
		}
	}

	pub fn to_notation(sq: Square) -> String {
		match sq {
			Self::A1 => String::from("a1"),
			Self::B1 => String::from("b1"),
			Self::C1 => String::from("c1"),
			Self::D1 => String::from("d1"),
			Self::E1 => String::from("e1"),
			Self::F1 => String::from("f1"),
			Self::G1 => String::from("g1"),
			Self::H1 => String::from("h1"),
			Self::A2 => String::from("a2"),
			Self::B2 => String::from("b2"),
			Self::C2 => String::from("c2"),
			Self::D2 => String::from("d2"),
			Self::E2 => String::from("e2"),
			Self::F2 => String::from("f2"),
			Self::G2 => String::from("g2"),
			Self::H2 => String::from("h2"),
			Self::A3 => String::from("a3"),
			Self::B3 => String::from("b3"),
			Self::C3 => String::from("c3"),
			Self::D3 => String::from("d3"),
			Self::E3 => String::from("e3"),
			Self::F3 => String::from("f3"),
			Self::G3 => String::from("g3"),
			Self::H3 => String::from("h3"),
			Self::A4 => String::from("a4"),
			Self::B4 => String::from("b4"),
			Self::C4 => String::from("c4"),
			Self::D4 => String::from("d4"),
			Self::E4 => String::from("e4"),
			Self::F4 => String::from("f4"),
			Self::G4 => String::from("g4"),
			Self::H4 => String::from("h4"),
			Self::A5 => String::from("a5"),
			Self::B5 => String::from("b5"),
			Self::C5 => String::from("c5"),
			Self::D5 => String::from("d5"),
			Self::E5 => String::from("e5"),
			Self::F5 => String::from("f5"),
			Self::G5 => String::from("g5"),
			Self::H5 => String::from("h5"),
			Self::A6 => String::from("a6"),
			Self::B6 => String::from("b6"),
			Self::C6 => String::from("c6"),
			Self::D6 => String::from("d6"),
			Self::E6 => String::from("e6"),
			Self::F6 => String::from("f6"),
			Self::G6 => String::from("g6"),
			Self::H6 => String::from("h6"),
			Self::A7 => String::from("a7"),
			Self::B7 => String::from("b7"),
			Self::C7 => String::from("c7"),
			Self::D7 => String::from("d7"),
			Self::E7 => String::from("e7"),
			Self::F7 => String::from("f7"),
			Self::G7 => String::from("g7"),
			Self::H7 => String::from("h7"),
			Self::A8 => String::from("a8"),
			Self::B8 => String::from("b8"),
			Self::C8 => String::from("c8"),
			Self::D8 => String::from("d8"),
			Self::E8 => String::from("e8"),
			Self::F8 => String::from("f8"),
			Self::G8 => String::from("g8"),
			Self::H8 => String::from("h8"),
			_ => panic!("Square not recognized, got: {sq}"),
		}
	}

	// The purpose of this function is to initialize the to_notation function
	#[cfg(debug_assertions)]
	pub fn _init_to_notation() {
		for r in Self::RANKS {
			for f in Self::FILES {
				let notation = format!("{}{}", FILES[f], r + 1);

				println!(
					r#"Self::{} => String::from("{notation}"),"#,
					notation.to_uppercase(),
				);
			}
		}
	}
}
