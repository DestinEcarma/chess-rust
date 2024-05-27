use crate::{
	bitboard::{Bitboard, Bitboards},
	piece::{Piece, Pieces},
	square::{I8File, I8Rank, I8Square, Square, Squares},
};

use super::MoveGenerator;

pub type Edge = usize;
pub type I8Edge = i8;

impl MoveGenerator {
	pub fn num_to_edge(sq: Square, piece: Piece) -> [Edge; 4] {
		let (r, f) = Squares::to_rank_file(sq);
		let r = r as I8Rank;
		let f = f as I8File;

		match piece {
			Pieces::BISHOP => {
				let ne = I8File::max(7 - f, 0) as Edge;
				let nn = I8Rank::max(7 - r, 0) as Edge;
				let nw = f as Edge;
				let ns = r as Edge;

				[
					Edge::min(nn, nw),
					Edge::min(nn, ne),
					Edge::min(ns, ne),
					Edge::min(ns, nw),
				]
			}
			Pieces::ROOK => [
				I8File::max(7 - f, 0) as Edge,
				I8Rank::max(7 - r, 0) as Edge,
				f as Edge,
				r as Edge,
			],
			_ => panic!("Piece not recognized, got: {piece}"),
		}
	}

	pub fn generate_sliding_post_mask(sq: Square, piece: Piece) -> Bitboard {
		let is_bishop = piece == Pieces::BISHOP;

		let num_to_edge = Self::num_to_edge(sq, piece);
		let offsets = match is_bishop {
			true => [7, 9, -7, -9],
			false => [1, 8, -1, -8],
		};

		let mut post_mask = Bitboards::EMPTY;
		let i8_sq = sq as I8Square;

		for direction in 0..4 {
			let offset = offsets[direction];
			let edge = I8Edge::max(num_to_edge[direction] as I8Edge, 1);

			for n in 1..edge {
				Bitboards::set_bit(&mut post_mask, (i8_sq + offset * n) as Square);
			}
		}

		post_mask
	}

	pub fn generate_sliding_attack_mask(sq: Square, piece: Piece, occ: Bitboard) -> Bitboard {
		let is_bishop = piece == Pieces::BISHOP;

		let num_to_edge = Self::num_to_edge(sq, piece);
		let offsets = match is_bishop {
			true => [7, 9, -7, -9],
			false => [1, 8, -1, -8],
		};

		let mut attack_mask = Bitboards::EMPTY;
		let i8_sq = sq as I8Square;

		for direction in 0..4 {
			let offset = offsets[direction];
			let edge = num_to_edge[direction] as I8Edge;

			for n in 1..=edge {
				let to = (i8_sq + offset * n) as Square;
				let to_bit = Bitboards::SQAURES[to];

				attack_mask |= to_bit;

				if Bitboards::occupied(occ, to) {
					break;
				}
			}
		}

		attack_mask
	}

	pub fn generate_blocker_boards(mask: Bitboard) -> Vec<Bitboard> {
		let mut blocker_boards = Vec::new();
		let mut current_mask = Bitboards::EMPTY;

		// Carry-Rippler
		// https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
		loop {
			blocker_boards.push(current_mask);
			current_mask = current_mask.wrapping_sub(mask) & mask;

			if current_mask == 0 {
				break;
			}
		}

		blocker_boards
	}

	pub fn generate_attack_boards(
		piece: Piece,
		sq: Square,
		blocker_boards: &[Bitboard],
	) -> Vec<Bitboard> {
		let mut attack_boards = Vec::new();

		for occ in blocker_boards {
			attack_boards.push(Self::generate_sliding_attack_mask(sq, piece, *occ));
		}

		attack_boards
	}
}
