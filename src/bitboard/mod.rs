mod miscs;

use crate::square::Square;

pub type Bitboard = usize;

pub struct Bitboards;
impl Bitboards {
	pub const EMPTY: Bitboard = 0;

	#[inline(always)]
	pub fn set_bit(bb: &mut Bitboard, sq: Square) {
		*bb |= Self::SQAURES[sq];
	}

	#[inline(always)]
	pub fn clear_bit(bb: &mut Bitboard, sq: Square) {
		*bb ^= Self::SQAURES[sq];
	}

	#[inline(always)]
	pub fn occupied(bb: Bitboard, sq: Square) -> bool {
		(bb & Self::SQAURES[sq]) != Self::EMPTY
	}

	#[inline(always)]
	pub fn get_lsb(bb: Bitboard) -> Square {
		bb.trailing_zeros() as Square
	}

	#[inline(always)]
	pub fn pop_lsb(bb: &mut Bitboard) -> Square {
		let sq = Self::get_lsb(*bb);
		*bb &= *bb - 1;

		sq
	}

	#[allow(dead_code)]
	pub fn print(bb: Bitboard, name: Option<&str>) {
		use crate::square::Squares;

		if let Some(name) = name {
			println!(" ↓ {name}\n");
		}

		let mut board = String::new();

		for r in Squares::RANKS.rev() {
			for f in Squares::FILES {
				let sq = Squares::from_rank_file(r, f);

				board += match Self::occupied(bb, sq) {
					true => " 1 ",
					false => " 0 ",
				}
			}

			board += "\n";
		}

		println!("{board}");
		println!(" ↑ Bitboard: {bb}\n");
	}
}
