use std::fmt::Display;

use crate::{
	bitboard::Bitboards,
	castle_right::CastleRights,
	color::Colors,
	piece::Pieces,
	square::Squares,
};

use super::Board;

impl Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let board_string = self.to_baord();
		let fen_string = self.to_fen();

		write!(f, "{}\nFen: {}", board_string, fen_string)
	}
}

impl Board {
	fn to_baord(&self) -> String {
		let mut board = String::from("   +———+———+———+———+———+———+———+———+\n");

		let white_pieces = self.bb_colors[Colors::WHITE];

		for r in Squares::RANKS.rev() {
			for f in Squares::FILES {
				if f == 0 {
					board += &format!(" {} ", r + 1);
				}

				let sq = Squares::from_rank_file(r, f);
				let piece = self.pieces[sq];
				let color = match Bitboards::occupied(white_pieces, sq) {
					true => Colors::WHITE,
					false => Colors::BLACK,
				};

				let char = match piece != Pieces::EMPTY {
					true => Pieces::to_string(piece, color),
					false => String::from(" "),
				};

				board += &format!("| {char} ");

				if f == (Squares::FILES.end - 1) {
					board += "|";
				}
			}

			board += "\n   +———+———+———+———+———+———+———+———+\n";
		}

		board += "     a   b   c   d   e   f   g   h\n";

		board
	}

	fn to_fen(&self) -> String {
		let mut board = String::new();
		let mut turn = String::new();
		let mut castle_rights = String::new();
		let mut en_passant = String::new();

		let white_pieces = self.bb_colors[Colors::WHITE];

		for r in Squares::RANKS.rev() {
			let mut empty = 0;

			for f in Squares::FILES {
				let sq = Squares::from_rank_file(r, f);
				let piece = self.pieces[sq];

				match piece == Pieces::EMPTY {
					true => empty += 1,
					false => {
						if empty != 0 {
							board += &empty.to_string();
						}

						let color = match Bitboards::occupied(white_pieces, sq) {
							true => Colors::WHITE,
							false => Colors::BLACK,
						};

						board += &Pieces::to_string(piece, color);
						empty = 0;
					}
				}
			}

			if empty != 0 {
				board += &empty.to_string();
			}

			if r != (Squares::RANKS.start) {
				board += "/";
			}
		}

		match self.game_state.color == Colors::WHITE {
			true => turn += "w",
			false => turn += "b",
		}

		match self.game_state.castle_rights > 0 {
			true => castle_rights += &CastleRights::to_string(self.game_state.castle_rights),
			false => castle_rights += "-",
		}

		match self.game_state.en_passant {
			Some(sq) => en_passant += &Squares::to_notation(sq),
			None => en_passant += "-",
		}

		format!("{board} {turn} {castle_rights} {en_passant}")
	}
}
