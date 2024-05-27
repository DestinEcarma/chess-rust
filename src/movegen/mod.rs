mod attacks;
pub mod defs;
mod init;
mod legal;
mod miscs;
pub mod movelist;
mod psuedo;

use crate::{bitboard::Bitboard, color::Colors, magic::Magic, square::Squares};

#[cfg_attr(debug, derive(Debug))]
pub struct MoveGenerator {
	king: [Bitboard; Squares::SIZE],
	pawn: [[Bitboard; Squares::SIZE]; Colors::SIZE],
	knight: [Bitboard; Squares::SIZE],
	bishop: Vec<Bitboard>,
	rook: Vec<Bitboard>,
	bishop_magic: [Magic; Squares::SIZE],
	rook_magic: [Magic; Squares::SIZE],
}

impl MoveGenerator {}
