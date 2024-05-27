use crate::{
	bitboard::{Bitboard, Bitboards},
	castle_right::CastleRight,
	color::{Color, Colors},
	piece::{Piece, Pieces},
	square::{Square, Squares},
};

use self::defs::GameState;

mod create;
pub mod defs;
mod display;
pub mod evaluation;
mod playmoves;

pub struct Board {
	pub bb_pieces: [[Bitboard; Colors::SIZE]; Pieces::SIZE],
	pub bb_colors: [Bitboard; Colors::SIZE],
	pub pieces: [Piece; Squares::SIZE],

	pub game_state: GameState,
	history: Vec<GameState>,
}

impl Board {
	#[inline(always)]
	pub fn color(&self) -> Color {
		self.game_state.color
	}

	#[inline(always)]
	pub fn castle_rights(&self) -> CastleRight {
		self.game_state.castle_rights
	}

	#[inline(always)]
	pub fn en_passant(&self) -> Option<Square> {
		self.game_state.en_passant
	}

	#[inline(always)]
	pub fn occupancy(&self) -> Bitboard {
		self.bb_colors[Colors::WHITE] | self.bb_colors[Colors::BLACK]
	}

	#[inline(always)]
	pub fn add_piece(&mut self, piece: Piece, color: Color, sq: Square) {
		Bitboards::set_bit(&mut self.bb_pieces[piece][color], sq);
		Bitboards::set_bit(&mut self.bb_colors[color], sq);
		self.pieces[sq] = piece;

		// self.game_state.material[color] += Pieces::VALUES[piece];
	}

	#[inline(always)]
	pub fn remove_piece(&mut self, piece: Piece, color: Color, sq: Square) {
		Bitboards::clear_bit(&mut self.bb_pieces[piece][color], sq);
		Bitboards::clear_bit(&mut self.bb_colors[color], sq);
		self.pieces[sq] = Pieces::EMPTY;

		// self.game_state.material[color] -= Pieces::VALUES[piece];
	}

	#[inline(always)]
	pub fn move_piece(&mut self, piece: Piece, color: Color, from: Square, to: Square) {
		self.remove_piece(piece, color, from);
		self.add_piece(piece, color, to);
	}

	#[inline(always)]
	pub fn reverse_move(&mut self, piece: Piece, color: Color, from: Square, to: Square) {
		self.remove_piece(piece, color, to);
		self.add_piece(piece, color, from);
	}
}
