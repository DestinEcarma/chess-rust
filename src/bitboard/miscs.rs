use crate::square::Squares;

use super::{Bitboard, Bitboards};

const fn _init_squares() -> [Bitboard; Squares::SIZE] {
	let mut bb_squares = [Bitboards::EMPTY; Squares::SIZE];

	let mut sq = Squares::A1;

	while sq < Squares::SIZE {
		bb_squares[sq] = 1 << sq;
		sq += 1;
	}

	bb_squares
}

impl Bitboards {
	pub const SQAURES: [Bitboard; Squares::SIZE] = _init_squares();

	#[cfg(debug)]
	pub fn print_indecies() {
		let mut indecies = String::new();

		for r in Squares::RANKS.rev() {
			for f in Squares::FILES {
				let sq = Squares::from_rank_file(r, f);

				indecies += &format!(" {:02} ", sq);
			}

			indecies += "\n";
		}

		println!("{indecies}");
	}
}
