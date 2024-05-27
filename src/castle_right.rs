use crate::{bitboard::Bitboard, square::Squares};

pub type CastleRight = usize;

const fn _init_castling_perms() -> [CastleRight; 64] {
	let mut cp = [CastleRights::ALL; 64];

	cp[Squares::H1] &= !CastleRights::WK;
	cp[Squares::A1] &= !CastleRights::WQ;
	cp[Squares::E1] &= !(CastleRights::WK | CastleRights::WQ);
	cp[Squares::H8] &= !CastleRights::BK;
	cp[Squares::A8] &= !CastleRights::BQ;
	cp[Squares::E8] &= !(CastleRights::BK | CastleRights::BQ);

	cp
}

pub struct CastleRights;
impl CastleRights {
	pub const WK: CastleRight = 0b0001;
	pub const WQ: CastleRight = 0b0010;
	pub const BK: CastleRight = 0b0100;
	pub const BQ: CastleRight = 0b1000;
	pub const ALL: CastleRight = 0b1111;
	pub const NONE: CastleRight = 0b0000;

	pub const ALL_RIGHTS: [CastleRight; 4] = [Self::WK, Self::WQ, Self::BK, Self::BQ];

	pub const WK_OCC: Bitboard = 0x60;
	pub const WQ_OCC: Bitboard = 0xe;
	pub const BK_OCC: Bitboard = 0x6000000000000000;
	pub const BQ_OCC: Bitboard = 0xe00000000000000;

	pub const PERMS: [CastleRight; Squares::SIZE] = _init_castling_perms();

	pub fn from_char(value: char) -> CastleRight {
		match value {
			'K' => Self::WK,
			'Q' => Self::WQ,
			'k' => Self::BK,
			'q' => Self::BQ,
			_ => panic!("Character not recognized, got: {value}"),
		}
	}

	pub fn to_string(value: CastleRight) -> String {
		let mut result = String::new();

		for mask in Self::ALL_RIGHTS {
			if (value & mask) != Self::NONE {
				result += match mask {
					Self::WK => "K",
					Self::WQ => "Q",
					Self::BK => "k",
					Self::BQ => "q",
					_ => panic!("Mask not recognized, got: {mask}"),
				}
			}
		}

		result
	}
}
