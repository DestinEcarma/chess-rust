use std::fmt::Display;

use crate::{
	piece::{Piece, Pieces},
	square::{Square, Squares},
};

pub struct Shift;
impl Shift {
	pub const PIECE: usize = 0;
	pub const FROM: usize = 3;
	pub const TO: usize = 9;
	pub const CAPTURED: usize = 15;
	pub const PROMOTION: usize = 18;
	pub const DOUBLE_PUSH: usize = 21;
	pub const EN_PASSANT: usize = 22;
	pub const CASTLING: usize = 23;

	pub const MOVE_INFO: usize = 21;
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Move {
	pub data: usize,
}

impl Display for Move {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let from = Squares::to_notation(self.from());
		let to = Squares::to_notation(self.to());
		let promotion = self.promotion();

		match promotion == Pieces::EMPTY {
			true => write!(f, "{from}{to}"),
			false => write!(f, "{from}{to}{}", Pieces::to_char(promotion)),
		}
	}
}

impl Move {
	pub const BOOL_MASK: usize = 0b1;
	pub const PIECE_MASK: usize = 0b111;
	pub const SQUARE_MASK: usize = 0b111111;
	pub const MOVE_INFO: usize = 0b111;

	pub const DOUBLE_PUSH: usize = 0b1;
	pub const EN_PASSANT: usize = 0b10;
	pub const CASTLING: usize = 0b100;

	#[inline(always)]
	pub fn piece(&self) -> Piece {
		self.data & Self::PIECE_MASK
	}

	#[inline(always)]
	pub fn from(&self) -> Square {
		(self.data >> Shift::FROM) & Self::SQUARE_MASK
	}

	#[inline(always)]
	pub fn to(&self) -> Square {
		(self.data >> Shift::TO) & Self::SQUARE_MASK
	}

	#[inline(always)]
	pub fn captured(&self) -> Piece {
		(self.data >> Shift::CAPTURED) & Self::PIECE_MASK
	}

	#[inline(always)]
	pub fn promotion(&self) -> Piece {
		(self.data >> Shift::PROMOTION) & Self::PIECE_MASK
	}

	#[inline(always)]
	pub fn move_info(&self) -> Piece {
		(self.data >> Shift::MOVE_INFO) & Self::MOVE_INFO
	}
}
