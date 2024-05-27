mod notations;

use std::ops::Range;

pub type Square = usize;
pub type Rank = usize;
pub type File = usize;

pub type I8Square = i8;
pub type I8Rank = i8;
pub type I8File = i8;

pub struct Squares;
impl Squares {
	pub const SIZE: usize = 64;
	pub const RANGE: Range<Square> = 0..64;
	pub const RANKS: Range<Rank> = 0..8;
	pub const FILES: Range<File> = 0..8;

	#[inline(always)]
	pub fn from_rank_file(r: Rank, f: File) -> Square {
		r * 8 + f
	}

	#[inline(always)]
	pub fn to_rank_file(sq: Square) -> (Rank, File) {
		let r = sq / 8;
		let f = sq - r * 8;

		(r, f)
	}
}
