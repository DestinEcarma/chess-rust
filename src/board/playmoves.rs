use crate::{
	bitboard::Bitboards,
	castle_right::CastleRights,
	movegen::{defs::Move, MoveGenerator},
	piece::Pieces,
	square::Squares,
};

use super::Board;

impl Board {
	#[inline(always)]
	pub fn make_move(&mut self, m: Move) {
		let mut current_gs = self.game_state;
		current_gs.next_move = m;
		self.history.push(current_gs);

		let color = self.color();
		let opp = color ^ 1;

		let piece = m.piece();
		let from = m.from();
		let to = m.to();
		let captured = m.captured();
		let promotion = m.promotion();
		let move_info = m.move_info();

		self.game_state.en_passant = None;

		if captured != Pieces::EMPTY {
			self.remove_piece(captured, opp, to);

			if captured == Pieces::ROOK {
				self.game_state.castle_rights &= CastleRights::PERMS[to];
			}
		}

		match piece != Pieces::PAWN {
			true => self.move_piece(piece, color, from, to),
			false => {
				self.remove_piece(piece, color, from);
				self.add_piece(
					match promotion != Pieces::EMPTY {
						true => promotion,
						false => piece,
					},
					color,
					to,
				);
			}
		}

		if move_info != 0 {
			match move_info {
				Move::DOUBLE_PUSH => self.game_state.en_passant = Some(to ^ 8),
				Move::EN_PASSANT => self.remove_piece(piece, opp, to ^ 8),
				Move::CASTLING => match to {
					Squares::G1 => self.move_piece(Pieces::ROOK, color, Squares::H1, Squares::F1),
					Squares::C1 => self.move_piece(Pieces::ROOK, color, Squares::A1, Squares::D1),
					Squares::G8 => self.move_piece(Pieces::ROOK, color, Squares::H8, Squares::F8),
					Squares::C8 => self.move_piece(Pieces::ROOK, color, Squares::A8, Squares::D8),
					_ => panic!("Error invalid castling right."),
				},
				_ => panic!("Move info not recognized, got: {move_info}"),
			}
		}

		if piece == Pieces::KING || piece == Pieces::ROOK {
			self.game_state.castle_rights &= CastleRights::PERMS[from]
		}

		self.game_state.color ^= 1;
	}

	#[inline(always)]
	pub fn make_psuedo_move(&mut self, m: Move, mg: &MoveGenerator) -> bool {
		let color = self.color();
		let opp = color ^ 1;

		self.make_move(m);

		let bb_king = self.bb_pieces[Pieces::KING][color];
		let king_sq = Bitboards::get_lsb(bb_king);
		let is_legal = !mg.square_attacked(self, opp, king_sq);

		if !is_legal {
			self.unmake();
		}

		is_legal
	}

	#[inline(always)]
	pub fn unmake(&mut self) {
		if let Some(gs) = self.history.pop() {
			self.game_state = gs;

			let color = self.color();
			let opp = color ^ 1;

			let m = self.game_state.next_move;
			let piece = m.piece();
			let from = m.from();
			let to = m.to();
			let captured = m.captured();
			let promotion = m.promotion();
			let move_info = m.move_info();

			match promotion == Pieces::EMPTY {
				true => self.reverse_move(piece, color, from, to),
				false => {
					self.remove_piece(promotion, color, to);
					self.add_piece(piece, color, from);
				}
			}

			if captured != Pieces::EMPTY {
				self.add_piece(captured, opp, to);
			}

			if move_info > 1 {
				match move_info {
					Move::EN_PASSANT => self.add_piece(Pieces::PAWN, opp, to ^ 8),
					Move::CASTLING => match to {
						Squares::G1 => {
							self.reverse_move(Pieces::ROOK, color, Squares::H1, Squares::F1)
						}
						Squares::C1 => {
							self.reverse_move(Pieces::ROOK, color, Squares::A1, Squares::D1)
						}
						Squares::G8 => {
							self.reverse_move(Pieces::ROOK, color, Squares::H8, Squares::F8)
						}
						Squares::C8 => {
							self.reverse_move(Pieces::ROOK, color, Squares::A8, Squares::D8)
						}
						_ => panic!("Error invalid reversing castle right."),
					},
					_ => panic!("Move info not recognized, got: {move_info}"),
				}
			}
		}
	}
}
