use std::str::FromStr;

use crate::{
	bitboard::Bitboards,
	castle_right::CastleRights,
	color::Colors,
	piece::Pieces,
	square::{File, Rank, Squares},
};

use super::{defs::GameState, Board};

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Debug)]
pub enum Error {
	InvalidFen(String),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Error::InvalidFen(value) => write!(f, "Invalid FEN: {}", value),
		}
	}
}

impl Default for Board {
	fn default() -> Self {
		Self::from_str(DEFAULT_FEN).unwrap()
	}
}

impl FromStr for Board {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let mut board = Self {
			bb_pieces: [[Bitboards::EMPTY; Colors::SIZE]; Pieces::SIZE],
			bb_colors: [Bitboards::EMPTY; Colors::SIZE],
			pieces: [Pieces::EMPTY; Squares::SIZE],

			game_state: GameState::new(),
			history: Vec::new(),
		};

		let tokens: Vec<&str> = value.split(' ').collect();

		if tokens.len() < 4 {
			return Err(Error::InvalidFen(value.to_string()));
		}

		let pieces = tokens[0];
		let turn = tokens[1];
		let castle_rights = tokens[2];
		let en_passant = tokens[3];

		let mut rank = 7 as Rank;
		let mut file = 0 as File;

		for ch in pieces.chars() {
			match ch {
				'/' => {
					rank -= 1;
					file = 0;
				}
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
					file += ch.to_digit(10).unwrap() as File
				}
				'K' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'k' | 'p' | 'n' | 'b' | 'r' | 'q' => {
					let sq = Squares::from_rank_file(rank, file);
					let color = Colors::from_char(ch);
					let piece = Pieces::from_char(ch);

					board.add_piece(piece, color, sq);
					file += 1;
				}
				_ => return Err(Error::InvalidFen(value.to_string())),
			}
		}

		match turn {
			"W" | "w" => board.game_state.color = Colors::WHITE,
			"B" | "b" => board.game_state.color = Colors::BLACK,
			_ => return Err(Error::InvalidFen(value.to_string())),
		}

		if castle_rights != "-" {
			for ch in castle_rights.chars() {
				board.game_state.castle_rights |= CastleRights::from_char(ch);
			}
		}

		if en_passant != "-" {
			board.game_state.en_passant = Some(Squares::from_notation(en_passant));
		}

		Ok(board)
	}
}
