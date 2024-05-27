use crate::{
	bitboard::Bitboard,
	board::Board,
	color::Color,
	piece::{Piece, Pieces},
	square::Square,
};

use super::MoveGenerator;

impl MoveGenerator {
	#[inline(always)]
	pub fn king_attacks(&self, sq: Square) -> Bitboard {
		self.king[sq]
	}

	#[inline(always)]
	pub fn pawn_attacks(&self, sq: Square, color: Color) -> Bitboard {
		self.pawn[color][sq]
	}

	#[inline(always)]
	pub fn knight_jumps(&self, sq: Square) -> Bitboard {
		self.knight[sq]
	}

	#[inline(always)]
	pub fn sliding_attacks(&self, piece: Piece, sq: Square, occ: Bitboard) -> Bitboard {
		match piece {
			Pieces::BISHOP => {
				let index = self.bishop_magic[sq].get_index(occ);
				self.bishop[index]
			}
			Pieces::ROOK => {
				let index = self.rook_magic[sq].get_index(occ);
				self.rook[index]
			}
			Pieces::QUEEN => {
				self.sliding_attacks(Pieces::BISHOP, sq, occ)
					| self.sliding_attacks(Pieces::ROOK, sq, occ)
			}
			_ => panic!("Piece not recognized, got: {piece}"),
		}
	}
}

impl MoveGenerator {
	#[inline(always)]
	pub fn square_attacked(&self, board: &Board, color: Color, sq: Square) -> bool {
		let occ = board.occupancy();
		let bb_pieces = &board.bb_pieces[..];

		bb_pieces[Pieces::KING][color] & self.king_attacks(sq) > 0
			|| bb_pieces[Pieces::PAWN][color] & self.pawn_attacks(sq, color ^ 1) > 0
			|| bb_pieces[Pieces::KNIGHT][color] & self.knight_jumps(sq) > 0
			|| bb_pieces[Pieces::BISHOP][color] & self.sliding_attacks(Pieces::BISHOP, sq, occ) > 0
			|| bb_pieces[Pieces::ROOK][color] & self.sliding_attacks(Pieces::ROOK, sq, occ) > 0
			|| bb_pieces[Pieces::QUEEN][color] & self.sliding_attacks(Pieces::QUEEN, sq, occ) > 0
	}
}
