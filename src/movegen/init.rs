use crate::{
	bitboard::{Bitboard, Bitboards},
	color::Colors,
	magic::{Magic, BISHOP_MAGIC_NR, ROOK_MAGIC_NR},
	piece::{Piece, Pieces},
	square::{File, I8File, I8Rank, I8Square, Square, Squares},
};

use super::MoveGenerator;

impl Default for MoveGenerator {
	fn default() -> Self {
		let mut movegen = Self {
			king: [Bitboards::EMPTY; Squares::SIZE],
			pawn: [[Bitboards::EMPTY; Squares::SIZE]; Colors::SIZE],
			knight: [Bitboards::EMPTY; Squares::SIZE],
			bishop: vec![Bitboards::EMPTY; 5_248],
			rook: vec![Bitboards::EMPTY; 102_400],
			bishop_magic: [Magic::default(); Squares::SIZE],
			rook_magic: [Magic::default(); Squares::SIZE],
		};

		movegen.init();
		movegen
	}
}

fn pawn_capture(capture: &mut Bitboard, sq: I8Square, f: File, left: i8, right: i8) {
	if f > 0 {
		Bitboards::set_bit(capture, (sq + left) as Square);
	}

	if f < 7 {
		Bitboards::set_bit(capture, (sq + right) as Square);
	}
}

impl MoveGenerator {
	fn init(&mut self) {
		self.init_king();
		self.init_pawn();
		self.init_knight();
		self.init_magic(Pieces::BISHOP);
		self.init_magic(Pieces::ROOK);
	}

	fn init_king(&mut self) {
		let king = &mut self.king[..];

		let diagonal_offsets = [7, 9, -7, -9];
		let rook_offsets = [1, 8, -1, -8];

		for sq in Squares::RANGE {
			let bishop_num_to_edge = Self::num_to_edge(sq, Pieces::BISHOP);
			let rook_num_to_edge = Self::num_to_edge(sq, Pieces::ROOK);

			let i8_sq = sq as I8Square;

			for direction in 0..4 {
				if bishop_num_to_edge[direction] > 0 {
					let to = (i8_sq + diagonal_offsets[direction]) as Square;
					Bitboards::set_bit(&mut king[sq], to);
				}

				if rook_num_to_edge[direction] > 0 {
					let to = (i8_sq + rook_offsets[direction]) as Square;
					Bitboards::set_bit(&mut king[sq], to);
				}
			}
		}
	}

	fn init_pawn(&mut self) {
		let pawn = &mut self.pawn[..];

		for sq in Squares::RANGE {
			let i8_sq = sq as I8Square;

			let (_, f) = Squares::to_rank_file(sq);

			if sq < 56 {
				pawn_capture(&mut pawn[Colors::WHITE][sq], i8_sq, f, 7, 9);
			}

			if sq >= 8 {
				pawn_capture(&mut pawn[Colors::BLACK][sq], i8_sq, f, -9, -7);
			}
		}
	}

	fn init_knight(&mut self) {
		let knight = &mut self.knight[..];

		let offsets = [17, 15, 10, 6, -6, -10, -15, -17];

		for sq in Squares::RANGE {
			let i8_sq = sq as I8Square;

			let (r, f) = Squares::to_rank_file(sq);
			let r = r as I8Rank;
			let f = f as I8File;

			for offset in offsets {
				let to = i8_sq + offset;

				if Squares::RANGE.contains(&(to as Square)) {
					let (to_r, to_f) = Squares::to_rank_file(to as Square);
					let to_r = to_r as I8Rank;
					let to_f = to_f as I8File;

					let max_distance = i8::max((f - to_f).abs(), (r - to_r).abs());

					if max_distance == 2 {
						knight[sq] |= Bitboards::SQAURES[to as Square];
					}
				}
			}
		}
	}

	fn init_magic(&mut self, piece: Piece) {
		let ok = piece == Pieces::BISHOP || piece == Pieces::ROOK;
		assert!(ok, "Illigal piece, got: {}", piece);

		let is_bishop = piece == Pieces::BISHOP;
		let mut offset = 0;

		let (magic_table, attack_table) = match is_bishop {
			true => (&mut self.bishop_magic[..], &mut self.bishop[..]),
			false => (&mut self.rook_magic[..], &mut self.rook[..]),
		};

		for sq in Squares::RANGE {
			let post_mask = Self::generate_sliding_post_mask(sq, piece);

			let bits = post_mask.count_ones();
			let permutations = 2usize.pow(bits);
			let end = offset + permutations - 1;

			let blocker_boards = Self::generate_blocker_boards(post_mask);
			let attack_boards = Self::generate_attack_boards(piece, sq, &blocker_boards);

			let magic = &mut magic_table[sq];

			magic.mask = post_mask;
			magic.shift = (64 - bits) as u8;
			magic.offset = offset;
			magic.nr = match is_bishop {
				true => BISHOP_MAGIC_NR[sq],
				false => ROOK_MAGIC_NR[sq],
			};

			for next in 0..permutations {
				let index = magic.get_index(blocker_boards[next]);

				match attack_table[index] == Bitboards::EMPTY {
					true => {
						let fail_low = index < offset;
						let fail_high = index > end;
						assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");

						attack_table[index] = attack_boards[next];
					}
					false => panic!("Attack table index not empty. Error in Magics."),
				}
			}

			offset += permutations;
		}
	}
}
