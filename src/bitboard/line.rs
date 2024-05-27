use crate::square::{I8File, I8Rank, I8Square, Square, Squares};

use super::{Bitboard, Bitboards};

type BitboardLine = [[Bitboard; Squares::SIZE]; Squares::SIZE];
type Edge = usize;
type NumToEdge = [Edge; 4];

const fn bishop_num_to_edge(sq: Square) -> NumToEdge {
	let r = (sq / 8) as I8Rank;
	let f = (sq as I8Square - r) as I8File;

	let _r = 7 - r;
	let _f = 7 - f;

	let nn = (if _r > 0 { _r } else { 0 }) as Edge;
	let ne = (if _f > 0 { _f } else { 0 }) as Edge;
	let ns = r as Edge;
	let nw = f as Edge;

	[
		if nn < nw { nn } else { nw },
		if nn < ne { nn } else { ne },
		if ns < ne { ns } else { ne },
		if ns < nw { ns } else { nw },
	]
}

const fn rook_num_to_edge(sq: Square) -> NumToEdge {
	let r = (sq / 8) as I8Rank;
	let f = (sq as I8Square - r * 8) as I8File;

	let _r = 7 - r;
	let _f = 7 - f;

	[
		if _f > 0 { _f } else { 0 } as Edge,
		if _r > 0 { _r } else { 0 } as Edge,
		f as Edge,
		r as Edge,
	]
}

const fn bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
	let offsets = [7, 9, -7, -9];

	let mut attack = Bitboards::EMPTY;

	let num_to_edge = bishop_num_to_edge(sq);
	let mut direction = 0;

	while direction < 4 {
		let offset = offsets[direction];
		let mut n = 1;

		while n <= num_to_edge[direction] as i8 {
			let _to = (sq as I8Square + offset * n) as Square;
			let to_bit = Bitboards::SQAURES[sq];
			attack |= to_bit;

			if (occ & to_bit) > 0 {
				break;
			}

			n += 1;
		}

		direction += 1;
	}

	attack
}

const fn rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
	let offsets = [1, 8, -1, -8];

	let mut attack = Bitboards::EMPTY;

	let num_to_edge = rook_num_to_edge(sq);
	let mut direction = 0;

	while direction < 4 {
		let offset = offsets[direction];
		let mut n = 1;

		while n <= num_to_edge[direction] as i8 {
			let _to = (sq as I8Square + offset * n) as Square;
			let to_bit = Bitboards::SQAURES[sq];
			attack |= to_bit;

			if (occ & to_bit) > 0 {
				break;
			}

			n += 1;
		}

		direction += 1;
	}

	attack
}

const fn _init_bb_line() -> BitboardLine {
	let mut bb_line = [[Bitboards::EMPTY; Squares::SIZE]; Squares::SIZE];

	let mut sq_1 = Squares::A1;

	while sq_1 < Squares::SIZE {
		let pseudo_attacks = [bishop_attacks(sq_1, 0), rook_attacks(sq_1, 0)];

		let sq_1_bit = Bitboards::SQAURES[sq_1];
		let mut sq_2 = Squares::A1;

		while sq_2 < Squares::SIZE {
			let bishop_attack_1 = pseudo_attacks[0];
			let rook_attack_1 = pseudo_attacks[1];
			let sq_2_bit = Bitboards::SQAURES[sq_2];

			if (bishop_attack_1 & sq_2_bit) > 0 {
				bb_line[sq_1][sq_2] =
					(bishop_attack_1 & bishop_attacks(sq_2, 0)) | sq_1_bit | sq_2_bit;
			}

			if (rook_attack_1 & sq_2_bit) > 0 {
				bb_line[sq_1][sq_2] = rook_attack_1 & rook_attacks(sq_2, 0) | sq_1_bit | sq_2_bit;
			}

			sq_2 += 1;
		}

		sq_1 += 1;
	}

	bb_line
}

const fn _init_bb_between() -> BitboardLine {
	let mut bb_between = [[Bitboards::EMPTY; Squares::SIZE]; Squares::SIZE];

	let mut sq_1 = Squares::A1;

	while sq_1 < Squares::SIZE {
		let sq_1_bit = Bitboards::SQAURES[sq_1];
		let mut sq_2 = Squares::A1;

		while sq_2 < Squares::SIZE {
			let sq_2_bit = Bitboards::SQAURES[sq_2];
			let bishop_attack_1 = bishop_attacks(sq_1, sq_2_bit);
			let rook_attack_1 = rook_attacks(sq_1, sq_2_bit);

			if (bishop_attack_1 & sq_2_bit) > 0 {
				bb_between[sq_1][sq_2] =
					(bishop_attack_1 & bishop_attacks(sq_2, sq_1_bit)) | sq_2_bit;
			}

			if (rook_attack_1 & sq_2_bit) > 0 {
				bb_between[sq_1][sq_2] = rook_attack_1 & rook_attacks(sq_2, sq_1_bit) | sq_2_bit;
			}

			sq_2 += 1;
		}

		sq_1 += 1;
	}

	bb_between
}

impl Bitboards {
	pub const BB_LINE: BitboardLine = _init_bb_line();
	pub const BB_BETWEEN: BitboardLine = _init_bb_between();

	#[inline(always)]
	pub fn aligned(from: Square, to: Square, bit: Bitboard) -> Bitboard {
		Self::BB_LINE[from][to] & bit
	}
}
