use crate::{
	bitboard::{Bitboard, Bitboards},
	board::Board,
	castle_right::CastleRights,
	color::Colors,
	piece::{Piece, Pieces},
	square::{I8Square, Square, Squares},
};

use super::{
	defs::{Move, Shift},
	movelist::MoveList,
	MoveGenerator,
};

impl MoveGenerator {
	pub fn generate_psuedo(&self, board: &Board) -> MoveList {
		let mut list = MoveList::new();

		self.king_moves(board, &mut list);
		self.pawn_moves(board, &mut list);
		self.knight_moves(board, &mut list);
		self.sliding_piece_moves(board, Pieces::BISHOP, &mut list);
		self.sliding_piece_moves(board, Pieces::ROOK, &mut list);
		self.sliding_piece_moves(board, Pieces::QUEEN, &mut list);

		list
	}
}

impl MoveGenerator {
	#[inline(always)]
	fn add_move(board: &Board, piece: Piece, from: Square, to: Bitboard, list: &mut MoveList) {
		let capture = board.pieces[to];

		let data = piece << Shift::PIECE
			| from << Shift::FROM
			| to << Shift::TO
			| capture << Shift::CAPTURED
			| Pieces::EMPTY << Shift::PROMOTION;

		list.push(Move { data });
	}

	#[inline(always)]
	fn add_moves(board: &Board, piece: Piece, from: Square, to: Bitboard, list: &mut MoveList) {
		let mut bb_moves = to;

		while bb_moves > 0 {
			let to = Bitboards::pop_lsb(&mut bb_moves);
			let capture = board.pieces[to];

			let data = piece << Shift::PIECE
				| from << Shift::FROM
				| to << Shift::TO
				| capture << Shift::CAPTURED
				| Pieces::EMPTY << Shift::PROMOTION;

			list.push(Move { data });
		}
	}

	#[inline(always)]
	fn promotion_move(board: &Board, from: Square, to: Square, list: &mut MoveList) {
		let dummy = Pieces::PAWN << Shift::PIECE
			| from << Shift::FROM
			| to << Shift::TO
			| board.pieces[to] << Shift::CAPTURED;

		for piece in Pieces::PROMOTIONAL {
			let data = dummy | piece << Shift::PROMOTION;

			list.push(Move { data });
		}
	}

	#[inline(always)]
	fn promotion_moves(board: &Board, from: Square, to: Bitboard, list: &mut MoveList) {
		let mut bb_moves = to;

		while bb_moves > 0 {
			let to = Bitboards::pop_lsb(&mut bb_moves);
			let dummy = Pieces::PAWN << Shift::PIECE
				| from << Shift::FROM
				| to << Shift::TO
				| board.pieces[to] << Shift::CAPTURED;

			for piece in Pieces::PROMOTIONAL {
				let data = dummy | piece << Shift::PROMOTION;

				list.push(Move { data });
			}
		}
	}

	#[inline(always)]
	fn add_special_move(piece: Piece, from: Square, to: Square, shift: usize, list: &mut MoveList) {
		let data = piece << Shift::PIECE
			| from << Shift::FROM
			| to << Shift::TO
			| Pieces::EMPTY << Shift::CAPTURED
			| Pieces::EMPTY << Shift::PROMOTION
			| Move::BOOL_MASK << shift;

		list.push(Move { data });
	}
}

impl MoveGenerator {
	#[inline(always)]
	fn king_moves(&self, board: &Board, list: &mut MoveList) {
		let color = board.color();

		let _occ = board.occupancy();
		let bb_not_ally = !board.bb_colors[color];
		let bb_king = board.bb_pieces[Pieces::KING][color];

		let sq = Bitboards::get_lsb(bb_king);
		let bb_attack = self.king[sq];

		let bb_moves = bb_attack & bb_not_ally;
		Self::add_moves(board, Pieces::KING, sq, bb_moves, list);
		self.castling(board, list);
	}

	#[inline(always)]
	fn castling(&self, board: &Board, list: &mut MoveList) {
		let color = board.color();
		let opp = color ^ 1;

		let occ = board.occupancy();
		let bb_king = board.bb_pieces[Pieces::KING][color];

		let sq = Bitboards::get_lsb(bb_king);
		let castle_rights = board.castle_rights();

		match color == Colors::WHITE {
			true => {
				if (castle_rights & CastleRights::WK != 0)
					&& (occ & CastleRights::WK_OCC == 0)
					&& !self.square_attacked(board, opp, Squares::E1)
					&& !self.square_attacked(board, opp, Squares::F1)
				{
					Self::add_special_move(Pieces::KING, sq, Squares::G1, Shift::CASTLING, list);
				}

				if (castle_rights & CastleRights::WQ != 0)
					&& (occ & CastleRights::WQ_OCC == 0)
					&& !self.square_attacked(board, opp, Squares::E1)
					&& !self.square_attacked(board, opp, Squares::D1)
				{
					Self::add_special_move(Pieces::KING, sq, Squares::C1, Shift::CASTLING, list);
				}
			}
			false => {
				if (castle_rights & CastleRights::BK != 0)
					&& (occ & CastleRights::BK_OCC == 0)
					&& !self.square_attacked(board, opp, Squares::E8)
					&& !self.square_attacked(board, opp, Squares::F8)
				{
					Self::add_special_move(Pieces::KING, sq, Squares::G8, Shift::CASTLING, list);
				}

				if (castle_rights & CastleRights::BQ != 0)
					&& (occ & CastleRights::BQ_OCC == 0)
					&& !self.square_attacked(board, opp, Squares::E8)
					&& !self.square_attacked(board, opp, Squares::D8)
				{
					Self::add_special_move(Pieces::KING, sq, Squares::C8, Shift::CASTLING, list);
				}
			}
		}
	}

	#[inline(always)]
	fn pawn_moves(&self, board: &Board, list: &mut MoveList) {
		let color = board.color();
		let opp = color ^ 1;

		let bb_empty = !board.occupancy();
		let bb_opp = board.bb_colors[opp];

		let (direction, promotion_rank, double_push_rank) = match color == Colors::WHITE {
			true => (8, 0xff00000000000000, 0xff000000),
			false => (-8, 0xff, 0xff00000000),
		};

		let rotation_count = (64 + direction) as u32;

		let mut bb_pawns = board.bb_pieces[Pieces::PAWN][color];

		while bb_pawns > 0 {
			let sq = Bitboards::pop_lsb(&mut bb_pawns);

			let single_push = (sq as I8Square + direction) as Square;

			let bb_single_push = Bitboards::SQAURES[single_push] & bb_empty;
			let bb_double_push =
				bb_single_push.rotate_left(rotation_count) & bb_empty & double_push_rank;

			if bb_single_push > 0 {
				match bb_single_push & promotion_rank != 0 {
					true => Self::promotion_move(board, sq, single_push, list),
					false => Self::add_move(board, Pieces::PAWN, sq, single_push, list),
				}
			}

			if bb_double_push > 0 {
				let to = Bitboards::get_lsb(bb_double_push);
				Self::add_special_move(Pieces::PAWN, sq, to, Shift::DOUBLE_PUSH, list)
			}

			let bb_attack = self.pawn_attacks(sq, color);
			let bb_moves = bb_attack & bb_opp;

			match bb_moves & promotion_rank != 0 {
				true => Self::promotion_moves(board, sq, bb_moves, list),
				false => Self::add_moves(board, Pieces::PAWN, sq, bb_moves, list),
			}

			if let Some(to) = board.en_passant() {
				if Bitboards::occupied(bb_attack, to) {
					Self::add_special_move(Pieces::PAWN, sq, to, Shift::EN_PASSANT, list)
				}
			}
		}
	}

	#[inline(always)]
	fn knight_moves(&self, board: &Board, list: &mut MoveList) {
		let color = board.color();

		let bb_not_ally = !board.bb_colors[color];
		let mut bb_knights = board.bb_pieces[Pieces::KNIGHT][color];

		while bb_knights > 0 {
			let sq = Bitboards::pop_lsb(&mut bb_knights);
			let bb_jumps = self.knight_jumps(sq);

			let bb_moves = bb_jumps & bb_not_ally;
			Self::add_moves(board, Pieces::KNIGHT, sq, bb_moves, list);
		}
	}

	#[inline(always)]
	fn sliding_piece_moves(&self, board: &Board, piece: Piece, list: &mut MoveList) {
		let color = board.color();

		let occ = board.occupancy();
		let bb_not_ally = !board.bb_colors[color];
		let mut bb_pieces = board.bb_pieces[piece][color];

		while bb_pieces > 0 {
			let sq = Bitboards::pop_lsb(&mut bb_pieces);
			let bb_attack = self.sliding_attacks(piece, sq, occ);

			let bb_moves = bb_attack & bb_not_ally;
			Self::add_moves(board, piece, sq, bb_moves, list);
		}
	}
}
