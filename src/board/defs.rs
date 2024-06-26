use crate::{
	castle_right::{CastleRight, CastleRights},
	color::{Color, Colors},
	movegen::defs::Move,
	square::Square,
};

#[derive(Clone, Copy)]
pub struct GameState {
	pub color: Color,

	pub castle_rights: CastleRight,
	pub en_passant: Option<Square>,

	pub next_move: Move,
}

impl GameState {
	pub fn new() -> Self {
		Self {
			color: Colors::WHITE,

			castle_rights: CastleRights::NONE,
			en_passant: None,

			next_move: Default::default(),
		}
	}
}
